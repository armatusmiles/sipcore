use sipmsg::*;

#[test]
fn parse_headers() {
    let parse_headers_result = SipHeaders::parse(
        "t: David <sip:davidko@biloxi.com>;tag=99sa0xk\r\n\
         From: caller <sip:caller2@example.com>;tag=323\r\n\
         Max-Forwards: 70\r\n\
         Call-ID: lwsdisp.1234abcd@funky.example.com\r\n\
         CSeq: 60 OPTIONS\r\n\
         e: tar\r\n\
         c: text/html; charset=ISO-8859-4\r\n\
         Content-Language: fr\r\n\
         ExtensionHeader: value;param=false\r\n\
         Authorization: Digest username=\"Alice\", realm=\"atlanta.com\" \r\n\
         \t,nonce=\"84a4cc6f3082121f32b42a2187831a9e\",\r\n \
         response=\"7587245234b3434cc3412213e5f113a5432\"\r\n\
         Content-Disposition: attachment; filename=smime.p7s; handling=required\r\n\
         l: 8\r\n\
         Proxy-Require: foo;boo\r\n\
         date: Sat, 15 Oct 2005 04:44:56 GMT\r\n\
         Expires: 5\r\n\
         Error-Info: <sip:not-in-service-recording@atlanta.com> \r\n\
         In-Reply-To: 70710@saturn.bell-tel.com, 17320@saturn.bell-tel.com\r\n\
         OrganizaTion: Boxes by Bob\r\n nextline\r\n\
         Priority: non-urgent\r\n\
         Proxy-Authenticate: Digest realm=\"atlanta.com\",\r\n \
         domain=\"sip:ss1.carrier.com\", qop=\"auth\", \r\n \
         nonce=\"f84f1cec41e6cbe5aea9c8e88d359\", \r\n \
         opaque=\"\", stale=FALSE, algorithm=MD5\r\n\
         Proxy-Authorization: Digest username=\"Alice\", realm=\"atlanta.com\", \r\n \
         nonce=\"c60f3082ee1212b402a21831ae\", \r\n \
         response=\"245f23415f11432b3434341c022\" \r\n\
         Record-Route: <sip:server10.biloxi.com;lr>,\r\n \
                    <sip:bigbox3.site3.atlanta.com;lr>\r\n\
         Route: <sip:alice@atlanta.com>,<sip:carol@chicago.com>\r\n\
         Reply-To: Bob <sip:bob@biloxi.com>\r\n\
         Require: 100rel\r\n\
         Retry-After: 18000 (I'm in a meeting) ;duration=3600\r\n\
         Server: FPBX-2.11.0(11.6.0)\r\n\
         S: I know you're there, pick up the phone and talk to me!\r\n\
         User-Agent: MxSipApp/4.4.11.69 MxSF/v3.2.7.30\r\n\
         k: 100rel\r\n\
         unsupported:   100rel\r\n\
         MIME-Version: 1.0\r\n\
         Min-Expires: 60\r\n\
         Timestamp: 54\r\n\
         WWW-Authenticate: Digest realm=\"atlanta.com\",\r\n \
            domain=\"sip:boxesbybob.com\", qop=\"auth\",\r\n \
            nonce=\"f84f1cec41e6cbe5aea9c8e88d359\",\r\n \
            opaque=\"\", stale=FALSE, algorithm=MD5\r\n\
         Warning: 301 isi.edu \"Incompatible network address type 'E.164'\"\r\n\
         V: SIP/2.0/UDP funky.example.com;branch=z9hG4bKkdjuw\r\n\r\nsomebody"
            .as_bytes(),
    );

    let (input, hdrs) = parse_headers_result.unwrap();
    assert_eq!(hdrs.len(), 37);

    let to_hdr = hdrs.get_rfc_s(SipRFCHeader::To).unwrap();
    assert_eq!(to_hdr.value.vstr, "David <sip:davidko@biloxi.com>");
    assert_eq!(to_hdr.params().unwrap().get(&"tag"), Some(&Some("99sa0xk")));
    assert_eq!(
        to_hdr.value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        to_hdr.value.sip_uri().unwrap().user_info().unwrap().value,
        "davidko"
    );

    assert_eq!(to_hdr.value.sip_uri().unwrap().hostport.host, "biloxi.com");

    let from_hdr = hdrs.get_rfc_s(SipRFCHeader::From).unwrap();
    assert_eq!(from_hdr.value.vstr, "caller <sip:caller2@example.com>");
    assert_eq!(from_hdr.params().unwrap().get(&"tag"), Some(&Some("323")));

    assert_eq!(
        from_hdr.value.tags().unwrap()[&SipHeaderTagType::DisplayName],
        b"caller"
    );
    assert_eq!(
        from_hdr.value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        from_hdr.value.sip_uri().unwrap().user_info().unwrap().value,
        "caller2"
    );
    assert_eq!(
        from_hdr.value.sip_uri().unwrap().hostport.host,
        "example.com"
    );

    let max_forwards_header = hdrs.get_rfc_s(SipRFCHeader::MaxForwards).unwrap();
    assert_eq!(max_forwards_header.value.vstr, "70");
    assert_eq!(max_forwards_header.params(), None);
    assert_eq!(max_forwards_header.value.vtype, SipHeaderValueType::Digit);

    assert_eq!(
        hdrs.get_rfc_s(SipRFCHeader::CallID).unwrap().value.vstr,
        "lwsdisp.1234abcd@funky.example.com"
    );
    assert_eq!(hdrs.get_rfc_s(SipRFCHeader::CallID).unwrap().params(), None);

    let cseq_header = hdrs.get_rfc_s(SipRFCHeader::CSeq).unwrap();
    assert_eq!(cseq_header.value.vstr, "60 OPTIONS");
    assert_eq!(hdrs.get_rfc_s(SipRFCHeader::CSeq).unwrap().params(), None);
    assert_eq!(
        cseq_header.value.tags().unwrap()[&SipHeaderTagType::Number],
        b"60"
    );
    assert_eq!(
        cseq_header.value.tags().unwrap()[&SipHeaderTagType::Method],
        b"OPTIONS"
    );

    assert_eq!(hdrs.get_ext_s("ExtensionHeader").unwrap().value.vstr, "value;param=false");

    let via_hdr = hdrs.get_rfc_s(SipRFCHeader::Via).unwrap();
    assert_eq!(via_hdr.value.vstr, "SIP/2.0/UDP funky.example.com");
    assert_eq!(
        via_hdr.params().unwrap().get(&"branch"),
        Some(&Some("z9hG4bKkdjuw"))
    );

    assert_eq!(
        via_hdr.value.tags().unwrap()[&SipHeaderTagType::ProtocolName],
        b"SIP"
    );
    assert_eq!(
        via_hdr.value.tags().unwrap()[&SipHeaderTagType::ProtocolVersion],
        b"2.0"
    );
    assert_eq!(
        via_hdr.value.tags().unwrap()[&SipHeaderTagType::ProtocolTransport],
        b"UDP"
    );
    assert_eq!(
        via_hdr.value.tags().unwrap()[&SipHeaderTagType::Host],
        b"funky.example.com"
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

    let content_disp_hdr = &hdrs.get_rfc_s(SipRFCHeader::ContentDisposition).unwrap();

    assert_eq!(content_disp_hdr.value.vstr, "attachment");
    assert_eq!(
        content_disp_hdr.params().unwrap().get("filename").unwrap(),
        &Some("smime.p7s")
    );
    assert_eq!(
        content_disp_hdr.params().unwrap().get("handling").unwrap(),
        &Some("required")
    );

    let content_language = &hdrs.get_rfc_s(SipRFCHeader::ContentLanguage).unwrap();
    assert_eq!(content_language.value.vstr, "fr");

    let content_encoding = &hdrs.get_rfc_s(SipRFCHeader::ContentEncoding).unwrap();
    assert_eq!(content_encoding.value.vstr, "tar");

    let content_length = &hdrs.get_rfc_s(SipRFCHeader::ContentLength).unwrap();
    assert_eq!(content_length.value.vstr, "8");

    let content_type = &hdrs.get_rfc_s(SipRFCHeader::ContentType).unwrap();
    assert_eq!(content_type.value.vstr, "text/html");
    assert_eq!(
        content_type.params().unwrap().get("charset").unwrap(),
        &Some("ISO-8859-4")
    );

    let date_hdr = &hdrs.get_rfc_s(SipRFCHeader::Date).unwrap();
    assert_eq!(date_hdr.value.vstr, "Sat, 15 Oct 2005 04:44:56 GMT");

    let error_info = &hdrs.get_rfc_s(SipRFCHeader::ErrorInfo).unwrap();
    assert_eq!(
        error_info.value.tags().unwrap()[&SipHeaderTagType::AbsoluteURI],
        "sip:not-in-service-recording@atlanta.com".as_bytes()
    );
    assert_eq!(
        error_info.value.vstr,
        "<sip:not-in-service-recording@atlanta.com>"
    );

    let expires_hdr = &hdrs.get_rfc_s(SipRFCHeader::Expires).unwrap();
    assert_eq!(expires_hdr.value.vstr, "5");

    let in_reply_hdrs = &hdrs.get_rfc(SipRFCHeader::InReplyTo).unwrap();
    assert_eq!(in_reply_hdrs[0].value.vstr, "70710@saturn.bell-tel.com");
    assert_eq!(
        in_reply_hdrs[0].value.tags().unwrap()[&SipHeaderTagType::ID],
        b"70710"
    );
    assert_eq!(
        in_reply_hdrs[0].value.tags().unwrap()[&SipHeaderTagType::Host],
        b"saturn.bell-tel.com"
    );
    assert_eq!(in_reply_hdrs[1].value.vstr, "17320@saturn.bell-tel.com");
    assert_eq!(
        in_reply_hdrs[1].value.tags().unwrap()[&SipHeaderTagType::ID],
        b"17320"
    );
    assert_eq!(
        in_reply_hdrs[1].value.tags().unwrap()[&SipHeaderTagType::Host],
        b"saturn.bell-tel.com"
    );

    let organization_header = &hdrs.get_rfc_s(SipRFCHeader::Organization).unwrap();
    assert_eq!(organization_header.value.vstr, "Boxes by Bob\r\n nextline");

    let priority_hdr = &hdrs.get_rfc_s(SipRFCHeader::Priority).unwrap();
    assert_eq!(priority_hdr.value.vstr, "non-urgent");

    let proxy_auth = &hdrs.get_rfc_s(SipRFCHeader::ProxyAuthenticate).unwrap();
    assert_eq!(
        proxy_auth.value.vstr,
        "Digest realm=\"atlanta.com\",\r\n \
    domain=\"sip:ss1.carrier.com\", qop=\"auth\", \r\n \
    nonce=\"f84f1cec41e6cbe5aea9c8e88d359\", \r\n \
    opaque=\"\", stale=FALSE, algorithm=MD5"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::AuthSchema],
        b"Digest"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Realm],
        b"atlanta.com"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Domain],
        b"sip:ss1.carrier.com"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::QopValue],
        b"auth"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Nonce],
        b"f84f1cec41e6cbe5aea9c8e88d359"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Opaque],
        b""
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Stale],
        b"FALSE"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Algorithm],
        b"MD5"
    );

    let proxy_auth = &hdrs.get_rfc_s(SipRFCHeader::ProxyAuthorization).unwrap();
    assert_eq!(
        proxy_auth.value.vstr,
        "Digest username=\"Alice\", realm=\"atlanta.com\", \r\n \
         nonce=\"c60f3082ee1212b402a21831ae\", \r\n \
         response=\"245f23415f11432b3434341c022\""
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::AuthSchema],
        b"Digest"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Username],
        b"Alice"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Nonce],
        b"c60f3082ee1212b402a21831ae"
    );
    assert_eq!(
        proxy_auth.value.tags().unwrap()[&SipHeaderTagType::Dresponse],
        b"245f23415f11432b3434341c022"
    );

    let proxy_require_hdr = &hdrs.get_rfc_s(SipRFCHeader::ProxyRequire).unwrap();
    assert_eq!(proxy_require_hdr.value.vstr, "foo");
    assert_eq!(proxy_require_hdr.params().unwrap().get("boo"), Some(&None));

    let record_route_headers = &hdrs.get_rfc(SipRFCHeader::RecordRoute).unwrap();

    assert_eq!(
        record_route_headers[0].value.vstr,
        "<sip:server10.biloxi.com;lr>"
    );
    assert_eq!(
        record_route_headers[0].value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        record_route_headers[0]
            .value
            .sip_uri()
            .unwrap()
            .hostport
            .host,
        "server10.biloxi.com"
    );
    assert_eq!(
        record_route_headers[1]
            .value
            .sip_uri()
            .unwrap()
            .params()
            .unwrap()
            .get("lr"),
        Some(&None)
    );
    assert_eq!(
        record_route_headers[1].value.vstr,
        "<sip:bigbox3.site3.atlanta.com;lr>"
    );
    assert_eq!(
        record_route_headers[1].value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        record_route_headers[1]
            .value
            .sip_uri()
            .unwrap()
            .hostport
            .host,
        "bigbox3.site3.atlanta.com"
    );
    assert_eq!(
        record_route_headers[1]
            .value
            .sip_uri()
            .unwrap()
            .params()
            .unwrap()
            .get("lr"),
        Some(&None)
    );

    let route_headers = &hdrs.get_rfc(SipRFCHeader::Route).unwrap();

    assert_eq!(route_headers[0].value.vstr, "<sip:alice@atlanta.com>");
    assert_eq!(
        route_headers[0].value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        route_headers[0].value.sip_uri().unwrap().hostport.host,
        "atlanta.com"
    );
    assert_eq!(
        route_headers[0]
            .value
            .sip_uri()
            .unwrap()
            .user_info()
            .unwrap()
            .value,
        "alice"
    );
    assert_eq!(route_headers[1].value.vstr, "<sip:carol@chicago.com>");
    assert_eq!(
        route_headers[1].value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        route_headers[1].value.sip_uri().unwrap().hostport.host,
        "chicago.com"
    );
    assert_eq!(
        route_headers[1]
            .value
            .sip_uri()
            .unwrap()
            .user_info()
            .unwrap()
            .value,
        "carol"
    );
    assert_eq!(route_headers[1].value.sip_uri().unwrap().params(), None);

    let reply_to_header = &hdrs.get_rfc_s(SipRFCHeader::ReplyTo).unwrap();
    assert_eq!(reply_to_header.value.vstr, "Bob <sip:bob@biloxi.com>");
    assert_eq!(
        reply_to_header.value.tags().unwrap()[&SipHeaderTagType::DisplayName],
        b"Bob"
    );
    assert_eq!(
        reply_to_header.value.sip_uri().unwrap().scheme,
        sipuri::RequestUriScheme::SIP
    );
    assert_eq!(
        reply_to_header
            .value
            .sip_uri()
            .unwrap()
            .user_info()
            .unwrap()
            .value,
        "bob"
    );
    assert_eq!(
        reply_to_header.value.sip_uri().unwrap().hostport.host,
        "biloxi.com"
    );

    let require_header = &hdrs.get_rfc_s(SipRFCHeader::Require).unwrap();
    assert_eq!(require_header.value.vstr, "100rel");

    let retry_after_hdr = &hdrs.get_rfc_s(SipRFCHeader::RetryAfter).unwrap();
    assert_eq!(retry_after_hdr.value.vstr, "18000 (I'm in a meeting)");
    assert_eq!(
        retry_after_hdr.value.tags().unwrap()[&SipHeaderTagType::Comment],
        "I'm in a meeting".as_bytes()
    );
    assert_eq!(
        retry_after_hdr.value.tags().unwrap()[&SipHeaderTagType::Seconds],
        "18000".as_bytes()
    );

    assert_eq!(
        retry_after_hdr.params().unwrap().get(&"duration"),
        Some(&Some("3600"))
    );

    let server_hdr = &hdrs.get_rfc_s(SipRFCHeader::Server).unwrap();
    assert_eq!(server_hdr.value.vstr, "FPBX-2.11.0(11.6.0)");

    let user_agent_hdr = &hdrs.get_rfc_s(SipRFCHeader::UserAgent).unwrap();
    assert_eq!(
        user_agent_hdr.value.vstr,
        "MxSipApp/4.4.11.69 MxSF/v3.2.7.30"
    );

    let subject_hdr = &hdrs.get_rfc_s(SipRFCHeader::Subject).unwrap();
    assert_eq!(
        subject_hdr.value.vstr,
        "I know you're there, pick up the phone and talk to me!"
    );

    let supported_hdr = &hdrs.get_rfc_s(SipRFCHeader::Supported).unwrap();
    assert_eq!(supported_hdr.value.vstr, "100rel");

    let unsupported_hdr = &hdrs.get_rfc_s(SipRFCHeader::Unsupported).unwrap();
    assert_eq!(unsupported_hdr.value.vstr, "100rel");

    let supported_hdr = &hdrs.get_rfc_s(SipRFCHeader::MimeVersion).unwrap();
    assert_eq!(supported_hdr.value.vstr, "1.0");

    assert_eq!(
        supported_hdr.value.tags().unwrap()[&SipHeaderTagType::Major],
        b"1"
    );
    assert_eq!(
        supported_hdr.value.tags().unwrap()[&SipHeaderTagType::Minor],
        b"0"
    );

    let min_exp_hdr = &hdrs.get_rfc_s(SipRFCHeader::MinExpires).unwrap();
    assert_eq!(min_exp_hdr.value.vstr, "60");

    let timestamp_hdr = &hdrs.get_rfc_s(SipRFCHeader::Timestamp).unwrap();
    assert_eq!(timestamp_hdr.value.vstr, "54");
    assert_eq!(
        timestamp_hdr.value.tags().unwrap()[&SipHeaderTagType::TimveVal],
        b"54"
    );

    let warn_hdr = &hdrs.get_rfc_s(SipRFCHeader::Warning).unwrap();
    assert_eq!(
        warn_hdr.value.vstr,
        "301 isi.edu \"Incompatible network address type 'E.164'\""
    );

    assert_eq!(
        warn_hdr.value.vstr,
        "301 isi.edu \"Incompatible network address type 'E.164'\""
    );
    assert_eq!(
        warn_hdr.value.tags().unwrap()[&SipHeaderTagType::WarnCode],
        b"301"
    );
    assert_eq!(
        warn_hdr.value.tags().unwrap()[&SipHeaderTagType::WarnAgent],
        b"isi.edu"
    );
    assert_eq!(
        warn_hdr.value.tags().unwrap()[&SipHeaderTagType::WarnText],
        "Incompatible network address type 'E.164'".as_bytes()
    );

    let www_auth = &hdrs.get_rfc_s(SipRFCHeader::WWWAuthenticate).unwrap();
    assert_eq!(
        www_auth.value.vstr,
        "Digest realm=\"atlanta.com\",\r\n \
        domain=\"sip:boxesbybob.com\", qop=\"auth\",\r\n \
        nonce=\"f84f1cec41e6cbe5aea9c8e88d359\",\r\n \
        opaque=\"\", stale=FALSE, algorithm=MD5"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::AuthSchema],
        b"Digest"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Realm],
        b"atlanta.com"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Domain],
        b"sip:boxesbybob.com"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::QopValue],
        b"auth"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Nonce],
        b"f84f1cec41e6cbe5aea9c8e88d359"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Opaque],
        b""
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Stale],
        b"FALSE"
    );
    assert_eq!(
        www_auth.value.tags().unwrap()[&SipHeaderTagType::Algorithm],
        b"MD5"
    );

    assert_eq!(input, "\r\nsomebody".as_bytes());
}
