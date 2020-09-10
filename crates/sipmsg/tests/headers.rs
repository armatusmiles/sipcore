use sipmsg::*;

#[test]
fn parse_headers() {
    let parse_headers_result = SipHeaders::parse(
        "To: sip:user@example.com\r\n\
         From: caller<sip:caller@example.com>;tag=323\r\n\
         Max-Forwards: 70\r\n\
         Call-ID: lwsdisp.1234abcd@funky.example.com\r\n\
         CSeq: 60 OPTIONS\r\n\
         CustomHeader: value;param=false\r\n\
         Authorization: Digest username=\"Alice\", realm=\"atlanta.com\" \r\n\
         \t,nonce=\"84a4cc6f3082121f32b42a2187831a9e\",\r\n \
         response=\"7587245234b3434cc3412213e5f113a5432\"\r\n\
         Via: SIP/2.0/UDP funky.example.com;branch=z9hG4bKkdjuw\r\n\r\nsomebody"
            .as_bytes(),
    );

    let (input, hdrs) = parse_headers_result.unwrap();
    assert_eq!(hdrs.len(), 8);
    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::To).unwrap().value.vstr,
        "sip:user@example.com"
    );
    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::From).unwrap().value.vstr,
        "caller<sip:caller@example.com>"
    );
    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::From)
            .unwrap()
            .params()
            .unwrap()
            .get(&"tag"),
        Some((&SipAscii::new("tag"), &Some("323")))
    );

    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::MaxForwards)
            .unwrap()
            .value
            .vstr,
        "70"
    );
    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::MaxForwards).unwrap().params(),
        None
    );

    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::CallID).unwrap().value.vstr,
        "lwsdisp.1234abcd@funky.example.com"
    );
    assert_eq!(hdrs.get_rfc_s(SipRFCHeader::CallID).unwrap().params(), None);

    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::CSeq).unwrap().value.vstr,
        "60 OPTIONS"
    );
    assert_eq!(hdrs.get_rfc_s(SipRFCHeader::CSeq).unwrap().params(), None);

    assert_eq!(hdrs.get_ext_s("customheader").unwrap().value.vstr, "value");
    assert_eq!(
        hdrs.get_ext_s("customheader")
            .unwrap()
            .params()
            .unwrap()
            .get(&"param"),
        Some((&SipAscii::new("param"), &Some("false")))
    );

    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::Via).unwrap().value.vstr,
        "SIP/2.0/UDP funky.example.com"
    );
    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::Via)
            .unwrap()
            .params()
            .unwrap()
            .get(&"branch"),
        Some((&SipAscii::new("branch"), &Some("z9hG4bKkdjuw")))
    );
    let auth_val = &hdrs.get_rfc_s(SipRFCHeader::Authorization).unwrap().value;
    assert_eq!(
        auth_val.vstr,
        "Digest username=\"Alice\", realm=\"atlanta.com\" \r\n\
        \t,nonce=\"84a4cc6f3082121f32b42a2187831a9e\",\r\n \
        response=\"7587245234b3434cc3412213e5f113a5432\""
    );
    assert_eq!(
        auth_val.tags().unwrap()[&SipHeaderTagType::Username],
        b"Alice"
    );
    assert_eq!(
        auth_val.tags().unwrap()[&SipHeaderTagType::Realm],
        b"atlanta.com"
    );
    assert_eq!(
        auth_val.tags().unwrap()[&SipHeaderTagType::Nonce],
        b"84a4cc6f3082121f32b42a2187831a9e"
    );
    assert_eq!(
        auth_val.tags().unwrap()[&SipHeaderTagType::Dresponse],
        "7587245234b3434cc3412213e5f113a5432".as_bytes()
    );
    assert_eq!(input, "\r\nsomebody".as_bytes());
}
