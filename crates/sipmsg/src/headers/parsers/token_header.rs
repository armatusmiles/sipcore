use crate::{
    common::{bnfcore::is_token_char, errorparse::SipParseError},
    headers::header::{HeaderValue, HeaderValueType},
};
use nom::bytes::complete::take_while1;

pub fn take(input: &[u8]) -> nom::IResult<&[u8], HeaderValue, SipParseError> {
    let (inp, res_val) = take_while1(is_token_char)(input)?;
    let (_, hdr_val) = HeaderValue::new(res_val, HeaderValueType::TokenValue, None, None)?;
    Ok((inp, hdr_val))
}
