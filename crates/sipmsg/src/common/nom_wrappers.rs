use crate::common::bnfcore::{is_crlf, is_escaped, is_wsp};
use crate::errorparse::SipParseError;
use core::str::from_utf8;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete,
    sequence::tuple,
};

pub fn take_while_with_escaped(
    input: &[u8],
    is_fun: fn(c: u8) -> bool,
) -> nom::IResult<&[u8], &[u8], SipParseError> {
    let mut idx = 0;
    while idx < input.len() {
        if is_fun(input[idx]) {
            idx += 1;
            continue;
        } else if is_escaped(&input[idx..]) {
            idx += 3;
            continue;
        }
        break;
    }

    Ok((&input[idx..], &input[..idx]))
}

/// LWS  =  [*WSP CRLF] 1*WSP ; linear whitespace
/// SWS  =  [LWS] ; sep whitespace
pub fn take_sws(source_input: &[u8]) -> nom::IResult<&[u8], &[u8], SipParseError> {
    let (input, _) = complete::space0(source_input)?; // *WSP
    if input.is_empty() || input.len() <= 2 {
        return Ok((input, b""));
    }
    if is_crlf(input) && (input.len() > 2 && is_wsp(input[2])) {
        let (input, _) = tag("\r\n")(input)?;
        return take_sws(input);
    }
    return Ok((input, b""));
}

/// trim start and end swses
/// assert_eq(take_while_trim_sws(" ab c", is_char), Ok(("ab", "c")));
/// assert_eq(take_while_trim_sws(" \r\n\tab c", is_char), Ok(("ab", "c")));
pub fn take_while_trim_sws(
    input: &[u8],
    cond_fun: fn(c: u8) -> bool,
) -> nom::IResult<&[u8], &[u8], SipParseError> {
    let (input, (_, result, _)) = tuple((take_sws, take_while1(cond_fun), take_sws))(input)?;
    Ok((input, result))
}

pub fn from_utf8_nom(v: &[u8]) -> nom::IResult<&str, &str, SipParseError> {
    match from_utf8(v) {
        Ok(res_str) => Ok(("", res_str)),
        Err(_) => sip_parse_error!(1, "Error: from_utf8_nom failed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bnfcore::*;

    fn test_sws_case(source_val: &str, expected_result: &str) {
        let res = take_sws(source_val.as_bytes());
        let (input, _) = res.unwrap();
        assert_eq!(input, expected_result.as_bytes());
    }

    #[test]
    fn test_sws_test() {
        test_sws_case("value", "value");
        test_sws_case("\r\nvalue", "\r\nvalue");
        test_sws_case("\r\n\tvalue", "value");
        test_sws_case("   \r\n\t \tvalue", "value");
        test_sws_case("  \r\nvalue", "\r\nvalue");
    }
    fn test_take_while_trim_sws_case(
        test_string: &str,
        expected_result: &str,
        expected_rest: &str,
    ) {
        let res = take_while_trim_sws(test_string.as_bytes(), is_token_char);
        let (input, result) = res.unwrap();
        assert_eq!(input, expected_rest.as_bytes());
        assert_eq!(result, expected_result.as_bytes());
    }

    #[test]
    fn test_take_while_trim_sws() {
        test_take_while_trim_sws_case(" qqq s", "qqq", "s");
        test_take_while_trim_sws_case("qqq s", "qqq", "s");
        test_take_while_trim_sws_case(" q ", "q", "");
        test_take_while_trim_sws_case("s", "s", "");
    }

    #[test]
    #[should_panic]
    fn test_take_while_trim_sws_panic() {
        test_take_while_trim_sws_case("", "", "");
    }

    fn take_while_with_escaped_test_case(
        input_str: &str,
        expected_res: &str,
        expected_rem: &str,
        cond_fun: fn(c: u8) -> bool,
    ) {
        let res = take_while_with_escaped(input_str.as_bytes(), cond_fun);
        let (remainder, result) = res.unwrap();
        assert_eq!(result, expected_res.as_bytes());
        assert_eq!(remainder, expected_rem.as_bytes());
    }

    #[test]
    fn take_while_with_escaped_test() {
        take_while_with_escaped_test_case(
            "project%20x&priority=urgent",
            "project%20x",
            "&priority=urgent",
            is_alpha,
        );
        take_while_with_escaped_test_case(
            "project%2Gx&priority=urgent",
            "project",
            "%2Gx&priority=urgent",
            is_alpha,
        );

        take_while_with_escaped_test_case("p", "p", "", is_alpha);
        take_while_with_escaped_test_case("123123X", "123123", "X", is_digit);
        take_while_with_escaped_test_case("abc", "", "abc", is_digit);
    }
}
