const SCRIPT_CASES: [(&str, &str); 8] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert('<script><\/scrIpt >');", "alert('<script></scrIpt >');"),
    (
        r"alert('<script><\/script>');alert('<script><\/script >');",
        "alert('<script></script>');alert('<script></script >');",
    ),
    (r"alert('<\!--');", "alert('<!--');"),
    (r"alert('<\!-- -->');alert('<\!--');", "alert('<!-- -->');alert('<!--');"),
    (r"alert('<script><\!--<\/script>');", "alert('<script><!--</script>');"),
    (r"alert('<\/script", "alert('</script"),
];

#[test]
fn encode_element_overlapping_candidates() {
    assert_eq!(r"<<\/script>", html_escape::encode_script("<</script>"));
    assert_eq!(r"<<\!--", html_escape::encode_script("<<!--"));
    assert_eq!(r"</scr<\/script>", html_escape::encode_script("</scr</script>"));
    assert_eq!(r"<\/script/>", html_escape::encode_script("</script/>"));
    assert_eq!(r"<<\/style>", html_escape::encode_style("<</style>"));
    assert_eq!(r"<<\!--", html_escape::encode_style("<<!--"));
    assert_eq!(r"</sty<\/style>", html_escape::encode_style("</sty</style>"));
    assert_eq!(r"<\/style/>", html_escape::encode_style("</style/>"));
}

#[test]
fn decode_element_requires_adjacent_escape_sequences() {
    assert_eq!(r"<abc\/script>", html_escape::decode_script(r"<abc\/script>"));
    assert_eq!(r"<abc\!--", html_escape::decode_script(r"<abc\!--"));
    assert_eq!(r"<\/scr</script>", html_escape::decode_script(r"<\/scr<\/script>"));
    assert_eq!(r"<</script>", html_escape::decode_script(r"<<\/script>"));
    assert_eq!(r"</script/>", html_escape::decode_script(r"<\/script/>"));
    assert_eq!(r"<abc\/style>", html_escape::decode_style(r"<abc\/style>"));
    assert_eq!(r"<abc\!--", html_escape::decode_style(r"<abc\!--"));
    assert_eq!(r"<\/sty</style>", html_escape::decode_style(r"<\/sty<\/style>"));
    assert_eq!(r"<</style>", html_escape::decode_style(r"<<\/style>"));
    assert_eq!(r"</style/>", html_escape::decode_style(r"<\/style/>"));
}

#[test]
fn encode_script() {
    for (expect, text) in SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_script(text));
    }
}

#[test]
fn encode_script_to_string() {
    for (expect, text) in SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_script_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_script_to_writer() {
    for (expect, text) in SCRIPT_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_script_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_script() {
    for (text, expect) in SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script(text));
    }
}

#[test]
fn decode_script_to_string() {
    for (text, expect) in SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_script_to_writer() {
    for (text, expect) in SCRIPT_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_script_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const SCRIPT_SINGLE_QUOTED_CASES: [(&str, &str); 24] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'\x3Cscript>\x3C/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        r#"alert("\x3Cscript>\x3C/script>");alert(\'\x3Cscript>\x3C/script >\');"#,
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"alert(\'\x3C!--\');", "alert('<!--');"),
    (r#"alert("\x3C!-- -->");alert(\'\x3C!--\');"#, "alert(\"<!-- -->\");alert('<!--');"),
    (r"\x3C\'/script>", "<'/script>"),
    (r"\'\x3C\'/script>", "'<'/script>"),
    (r"\x3C/scri\'pt>", "</scri'pt>"),
    (r"\'\x3C/scri\'pt>", "'</scri'pt>"),
    (r"\x3C\'!--", "<'!--"),
    (r"\'\x3C\'!--", "'<'!--"),
    (r"\x3C!-\'-", "<!-'-"),
    (r"\'\x3C!-\'-", "'<!-'-"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\x3CA", "<A"),
    (r"\x3C A", "< A"),
    (r"\x0D\x0A\x0C", "\r\n\u{000C}"),
    (r"中文\u2028\u2029🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\x0A", r"\x0A"),
    (r"\x3C\\/script>", r"<\/script>"),
    (r"\x00", "\0"),
    (r#"\\\'""#, r#"\'""#),
];

#[test]
fn encode_script_single_quoted_text() {
    for (expect, text) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_script_single_quoted_text(text));
    }
}

#[test]
fn encode_script_single_quoted_text_to_string() {
    for (expect, text) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_script_single_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_script_single_quoted_text_to_writer() {
    for (expect, text) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_script_single_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_script_single_quoted_text() {
    for (text, expect) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script_single_quoted_text(text));
    }
}

#[test]
fn decode_script_single_quoted_text_to_string() {
    for (text, expect) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::decode_script_single_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_script_single_quoted_text_to_writer() {
    for (text, expect) in SCRIPT_SINGLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_script_single_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const SCRIPT_DOUBLE_QUOTED_CASES: [(&str, &str); 14] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r#"alert(\"\x3Cscript>\x3C/script>\");"#, "alert(\"<script></script>\");"),
    (r#"alert(\"\x3C!--\");"#, "alert(\"<!--\");"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\x3CA", "<A"),
    (r"\x3C A", "< A"),
    (r"\x0D\x0A\x0C", "\r\n\u{000C}"),
    (r"中文\u2028\u2029🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\x0A", r"\x0A"),
    (r"\x3C\\/script>", r"<\/script>"),
    (r"\x00", "\0"),
    (r#"\\'\""#, r#"\'""#),
];

#[test]
fn encode_script_double_quoted_text() {
    for (expect, text) in SCRIPT_DOUBLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_script_double_quoted_text(text));
    }
}

#[test]
fn decode_script_double_quoted_text() {
    for (text, expect) in SCRIPT_DOUBLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script_double_quoted_text(text));
    }
}

#[test]
fn element_to_vec_appends_and_returns_written_data() {
    let mut encoded = b"prefix:".to_vec();

    assert_eq!(
        r#"\"\x3Cscript>\x3C/script>\""#.as_bytes(),
        html_escape::encode_script_double_quoted_text_to_vec(
            r#""<script></script>""#,
            &mut encoded
        )
    );
    assert_eq!(br#"prefix:\"\x3Cscript>\x3C/script>\""#, encoded.as_slice());

    let mut decoded = b"prefix:".to_vec();

    assert_eq!(
        r#""<script></script>""#.as_bytes(),
        html_escape::decode_script_double_quoted_text_to_vec(
            r#"\"<script><\/script>\""#,
            &mut decoded
        )
    );
    assert_eq!(br#"prefix:"<script></script>""#, decoded.as_slice());
}

const SCRIPT_QUOTED_CASES: [(&str, &str); 17] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'\x3Cscript>\x3C/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        r#"alert(\"\x3Cscript>\x3C/script>\");alert(\'\x3Cscript>\x3C/script >\');"#,
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"\x3C/script>1\'2\'3", "</script>1'2'3"),
    (r"alert(\'\x3C!--\');", "alert('<!--');"),
    (r#"alert(\"\x3C!-- -->\");alert(\'\x3C!--\');"#, "alert(\"<!-- -->\");alert('<!--');"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\x3CA", "<A"),
    (r"\x3C A", "< A"),
    (r"\x0D\x0A\x0C", "\r\n\u{000C}"),
    (r"中文\u2028\u2029🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\x0A", r"\x0A"),
    (r"\x3C\\/script>", r"<\/script>"),
    (r"\x00", "\0"),
    (r#"\\\'\""#, r#"\'""#),
];

#[test]
fn encode_script_quoted_text() {
    for (expect, text) in SCRIPT_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_script_quoted_text(text));
    }
}

#[test]
fn encode_script_quoted_text_to_string() {
    for (expect, text) in SCRIPT_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_script_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_script_quoted_text_to_writer() {
    for (expect, text) in SCRIPT_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_script_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_script_quoted_text() {
    for (text, expect) in SCRIPT_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script_quoted_text(text));
    }
}

#[test]
fn decode_script_quoted_text_to_string() {
    for (text, expect) in SCRIPT_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::decode_script_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_script_quoted_text_to_writer() {
    for (text, expect) in SCRIPT_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_script_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const STYLE_CASES: [(&str, &str); 8] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"div::after { content: '<style><\/stYle >';}", "div::after { content: '<style></stYle >';}"),
    (
        r"div::after { content: '<style><\/style>';} label::after { content: '<style><\/style >';}",
        "div::after { content: '<style></style>';} label::after { content: '<style></style >';}",
    ),
    (r"div::after { content: '<\!--';}", "div::after { content: '<!--';}"),
    (
        r"div::after { content: '<\!-- -->';} label::after { content: '<\!--';}",
        "div::after { content: '<!-- -->';} label::after { content: '<!--';}",
    ),
    (
        r"div::after { content: '<style><\!--<\/style>';}",
        "div::after { content: '<style><!--</style>';}",
    ),
    (r"div::after { content: '<\/style", "div::after { content: '</style"),
];

#[test]
fn encode_style() {
    for (expect, text) in STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_style(text));
    }
}

#[test]
fn encode_style_to_string() {
    for (expect, text) in STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_style_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_style_to_writer() {
    for (expect, text) in STYLE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_style_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_style() {
    for (text, expect) in STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style(text));
    }
}

#[test]
fn decode_style_to_string() {
    for (text, expect) in STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_style_to_writer() {
    for (text, expect) in STYLE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_style_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const STYLE_SINGLE_QUOTED_CASES: [(&str, &str); 23] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r"div::after { content: \'\3C style>\3C /stYle >\';}",
        "div::after { content: '<style></stYle >';}",
    ),
    (
        r#"div::after { content: "\3C style>\3C /style>";} label::after { content: \'\3C style>\3C /style >\';}"#,
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"alert(\'\3C !--\');", "alert('<!--');"),
    (r#"alert("\3C !-- -->");alert(\'\3C !--\');"#, "alert(\"<!-- -->\");alert('<!--');"),
    (r"\3C \'/style>", "<'/style>"),
    (r"\'\3C \'/style>", "'<'/style>"),
    (r"\3C /sty\'le>", "</sty'le>"),
    (r"\'\3C /sty\'le>", "'</sty'le>"),
    (r"\3C \'!--", "<'!--"),
    (r"\'\3C \'!--", "'<'!--"),
    (r"\3C !-\'-", "<!-'-"),
    (r"\'\3C !-\'-", "'<!-'-"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\3C A", "<A"),
    (r"\3C  A", "< A"),
    (r"\0D \0A \0C ", "\r\n\u{000C}"),
    ("中文\u{2028}\u{2029}🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\0A ", r"\0A "),
    (r"\3C \\/style>", r"<\/style>"),
    (r#"\\\'""#, r#"\'""#),
];

#[test]
fn encode_style_single_quoted_text() {
    for (expect, text) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_style_single_quoted_text(text));
    }
}

#[test]
fn encode_style_single_quoted_text_to_string() {
    for (expect, text) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_style_single_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_style_single_quoted_text_to_writer() {
    for (expect, text) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_style_single_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_style_single_quoted_text() {
    for (text, expect) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style_single_quoted_text(text));
    }
}

#[test]
fn decode_style_single_quoted_text_to_string() {
    for (text, expect) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::decode_style_single_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_style_single_quoted_text_to_writer() {
    for (text, expect) in STYLE_SINGLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_style_single_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const STYLE_DOUBLE_QUOTED_CASES: [(&str, &str); 13] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r#"div::after { content: \"\3C style>\3C /style>\";}"#,
        "div::after { content: \"<style></style>\";}",
    ),
    (r#"div::after { content: \"\3C !--\";}"#, "div::after { content: \"<!--\";}"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\3C A", "<A"),
    (r"\3C  A", "< A"),
    (r"\0D \0A \0C ", "\r\n\u{000C}"),
    ("中文\u{2028}\u{2029}🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\0A ", r"\0A "),
    (r"\3C \\/style>", r"<\/style>"),
    (r#"\\'\""#, r#"\'""#),
];

#[test]
fn encode_style_double_quoted_text() {
    for (expect, text) in STYLE_DOUBLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_style_double_quoted_text(text));
    }
}

#[test]
fn decode_style_double_quoted_text() {
    for (text, expect) in STYLE_DOUBLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style_double_quoted_text(text));
    }
}

const STYLE_QUOTED_CASES: [(&str, &str); 16] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r"div::after { content: \'\3C style>\3C /stYle >\';}",
        "div::after { content: '<style></stYle >';}",
    ),
    (
        r#"div::after { content: \"\3C style>\3C /style>\";} label::after { content: \'\3C style>\3C /style >\';}"#,
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"\3C /style>1\'2\'3", "</style>1'2'3"),
    (r"alert(\'\3C !--\');", "alert('<!--');"),
    (r#"alert(\"\3C !-- -->\");alert(\'\3C !--\');"#, "alert(\"<!-- -->\");alert('<!--');"),
    (r"tail\\", r"tail\"),
    (r"\\n", r"\n"),
    (r"\3C A", "<A"),
    (r"\3C  A", "< A"),
    (r"\0D \0A \0C ", "\r\n\u{000C}"),
    ("中文\u{2028}\u{2029}🙂", "中文\u{2028}\u{2029}🙂"),
    (r"\\0A ", r"\0A "),
    (r"\3C \\/style>", r"<\/style>"),
    (r#"\\\'\""#, r#"\'""#),
];

#[test]
fn encode_style_quoted_text() {
    for (expect, text) in STYLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_style_quoted_text(text));
    }
}

#[test]
fn encode_style_quoted_text_to_string() {
    for (expect, text) in STYLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_style_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_style_quoted_text_to_writer() {
    for (expect, text) in STYLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_style_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_style_quoted_text() {
    for (text, expect) in STYLE_QUOTED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style_quoted_text(text));
    }
}

#[test]
fn decode_style_quoted_text_to_string() {
    for (text, expect) in STYLE_QUOTED_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::decode_style_quoted_text_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_style_quoted_text_to_writer() {
    for (text, expect) in STYLE_QUOTED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_style_quoted_text_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn quoted_text_escapes_each_chunk() {
    for parts in [["</scr", "ipt>"], ["<!", "--"], ["<", " A\\"], ["line1\r", "\nline2"]] {
        let text = parts.concat();
        let mut script = String::new();
        let mut style = String::new();

        for part in parts {
            html_escape::encode_script_quoted_text_to_string(part, &mut script);
            html_escape::encode_style_quoted_text_to_string(part, &mut style);
        }

        assert_eq!(html_escape::encode_script_quoted_text(&text), script);
        assert_eq!(html_escape::encode_style_quoted_text(&text), style);
        assert_eq!(text, html_escape::decode_script_quoted_text(&script));
        assert_eq!(text, html_escape::decode_style_quoted_text(&style));

        #[cfg(feature = "std")]
        {
            let mut script_writer = Vec::new();
            let mut style_writer = Vec::new();

            for part in parts {
                html_escape::encode_script_quoted_text_to_writer(part, &mut script_writer).unwrap();
                html_escape::encode_style_quoted_text_to_writer(part, &mut style_writer).unwrap();
            }

            assert_eq!(script.as_bytes(), script_writer.as_slice());
            assert_eq!(style.as_bytes(), style_writer.as_slice());
        }
    }
}

#[test]
fn decode_quoted_text_accepts_legacy_escapes_without_decoding_again() {
    for (expect, text) in [
        (r"<\/script>", r"\x3C\/script>"),
        ("'<</script>", r"\'<<\/script>"),
        (r"<\!--", r"\x3C\!--"),
        ("<!--", r"<\!--"),
    ] {
        assert_eq!(expect, html_escape::decode_script_quoted_text(text));
        assert_eq!(
            expect.as_bytes(),
            html_escape::decode_script_quoted_text_to_vec(text, &mut Vec::new())
        );

        #[cfg(feature = "std")]
        {
            let mut output = Vec::new();
            html_escape::decode_script_quoted_text_to_writer(text, &mut output).unwrap();
            assert_eq!(expect.as_bytes(), output.as_slice());
        }
    }

    for (expect, text) in [
        (r"<\/style>", r"\3C \/style>"),
        ("'<</style>", r"\'<<\/style>"),
        (r"<\!--", r"\3C \!--"),
        ("<!--", r"<\!--"),
    ] {
        assert_eq!(expect, html_escape::decode_style_quoted_text(text));
        assert_eq!(
            expect.as_bytes(),
            html_escape::decode_style_quoted_text_to_vec(text, &mut Vec::new())
        );

        #[cfg(feature = "std")]
        {
            let mut output = Vec::new();
            html_escape::decode_style_quoted_text_to_writer(text, &mut output).unwrap();
            assert_eq!(expect.as_bytes(), output.as_slice());
        }
    }
}

#[test]
fn style_quoted_text_replaces_null() {
    assert_eq!("a\u{FFFD}b", html_escape::encode_style_single_quoted_text("a\0b"));
    assert_eq!("a\u{FFFD}b", html_escape::encode_style_double_quoted_text("a\0b"));
    assert_eq!("a\u{FFFD}b", html_escape::encode_style_quoted_text("a\0b"));

    let mut output = String::from("prefix:");
    assert_eq!("a\u{FFFD}b", html_escape::encode_style_quoted_text_to_string("a\0b", &mut output));
    assert_eq!("prefix:a\u{FFFD}b", output);
    assert_eq!("a\u{FFFD}b", html_escape::decode_style_quoted_text("a\u{FFFD}b"));

    #[cfg(feature = "std")]
    {
        let mut output = Vec::new();
        html_escape::encode_style_quoted_text_to_writer("a\0b", &mut output).unwrap();
        assert_eq!("a\u{FFFD}b".as_bytes(), output.as_slice());
    }
}
