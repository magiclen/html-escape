const SCRIPT_CASES: [(&str, &str); 7] = [
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

const SCRIPT_SINGLE_QUOTED_CASES: [(&str, &str); 14] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'<script><\/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        "alert(\"<script><\\/script>\");alert(\\'<script><\\/script >\\');",
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"alert(\'<\!--\');", "alert('<!--');"),
    ("alert(\"<\\!-- -->\");alert(\\'<\\!--\\');", "alert(\"<!-- -->\");alert('<!--');"),
    (r"<\'/script>", "<'/script>"),
    (r"\'<\'/script>", "'<'/script>"),
    (r"</scri\'pt>", "</scri'pt>"),
    (r"\'</scri\'pt>", "'</scri'pt>"),
    (r"<\'!--", "<'!--"),
    (r"\'<\'!--", "'<'!--"),
    (r"<!-\'-", "<!-'-"),
    (r"\'<!-\'-", "'<!-'-"),
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

const SCRIPT_DOUBLE_QUOTED_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r#"alert(\"<script><\/script>\");"#, r#"alert("<script></script>");"#),
    (r#"alert(\"<\!--\");"#, r#"alert("<!--");"#),
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
        r#"\"<script><\/script>\""#.as_bytes(),
        html_escape::encode_script_double_quoted_text_to_vec(
            r#""<script></script>""#,
            &mut encoded
        )
    );
    assert_eq!(br#"prefix:\"<script><\/script>\""#, encoded.as_slice());

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

const SCRIPT_QUOTED_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'<script><\/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        "alert(\\\"<script><\\/script>\\\");alert(\\'<script><\\/script >\\');",
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"<\/script>1\'2\'3", "</script>1'2'3"),
    (r"alert(\'<\!--\');", "alert('<!--');"),
    ("alert(\\\"<\\!-- -->\\\");alert(\\'<\\!--\\');", "alert(\"<!-- -->\");alert('<!--');"),
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

const STYLE_CASES: [(&str, &str); 7] = [
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

const STYLE_SINGLE_QUOTED_CASES: [(&str, &str); 14] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r"div::after { content: \'<style><\/stYle >\';}",
        "div::after { content: '<style></stYle >';}",
    ),
    (
        "div::after { content: \"<style><\\/style>\";} label::after { content: \
         \\'<style><\\/style >\\';}",
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"alert(\'<\!--\');", "alert('<!--');"),
    ("alert(\"<\\!-- -->\");alert(\\'<\\!--\\');", "alert(\"<!-- -->\");alert('<!--');"),
    (r"<\'/style>", "<'/style>"),
    (r"\'<\'/style>", "'<'/style>"),
    (r"</sty\'le>", "</sty'le>"),
    (r"\'</sty\'le>", "'</sty'le>"),
    (r"<\'!--", "<'!--"),
    (r"\'<\'!--", "'<'!--"),
    (r"<!-\'-", "<!-'-"),
    (r"\'<!-\'-", "'<!-'-"),
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

const STYLE_DOUBLE_QUOTED_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r#"div::after { content: \"<style><\/style>\";}"#,
        r#"div::after { content: "<style></style>";}"#,
    ),
    (r#"div::after { content: \"<\!--\";}"#, r#"div::after { content: "<!--";}"#),
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

const STYLE_QUOTED_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (
        r"div::after { content: \'<style><\/stYle >\';}",
        "div::after { content: '<style></stYle >';}",
    ),
    (
        "div::after { content: \\\"<style><\\/style>\\\";} label::after { content: \
         \\'<style><\\/style >\\';}",
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"<\/style>1\'2\'3", "</style>1'2'3"),
    (r"alert(\'<\!--\');", "alert('<!--');"),
    ("alert(\\\"<\\!-- -->\\\");alert(\\'<\\!--\\');", "alert(\"<!-- -->\");alert('<!--');"),
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
