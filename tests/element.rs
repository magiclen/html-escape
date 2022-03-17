const SCRIPT_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert('<script><\/scrIpt >');", "alert('<script></scrIpt >');"),
    (
        r"alert('<script><\/script>');alert('<script><\/script >');",
        "alert('<script></script>');alert('<script></script >');",
    ),
];

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

const SCRIPT_SINGLE_QUOTED_CASES: [(&str, &str); 8] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'<script><\/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        "alert(\"<script><\\/script>\");alert(\\'<script><\\/script >\\');",
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"<\'/script>", "<'/script>"),
    (r"\'<\'/script>", "'<'/script>"),
    (r"</scri\'pt>", "</scri'pt>"),
    (r"\'</scri\'pt>", "'</scri'pt>"),
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

const SCRIPT_QUOTED_CASES: [(&str, &str); 5] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert(\'<script><\/scrIpt >\');", "alert('<script></scrIpt >');"),
    (
        "alert(\\\"<script><\\/script>\\\");alert(\\'<script><\\/script >\\');",
        "alert(\"<script></script>\");alert('<script></script >');",
    ),
    (r"<\/script>1\'2\'3", "</script>1'2'3"),
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

const STYLE_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"div::after { content: '<style><\/stYle >';}", "div::after { content: '<style></stYle >';}"),
    (
        r"div::after { content: '<style><\/style>';} label::after { content: '<style><\/style >';}",
        "div::after { content: '<style></style>';} label::after { content: '<style></style >';}",
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

const STYLE_SINGLE_QUOTED_CASES: [(&str, &str); 8] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"div::after { content: \'<style><\/stYle >\';}", "div::after { content: '<style></stYle >';}"),
    (
        "div::after { content: \"<style><\\/style>\";} label::after { content: \\'<style><\\/style >\\';}",
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"<\'/style>", "<'/style>"),
    (r"\'<\'/style>", "'<'/style>"),
    (r"</sty\'le>", "</sty'le>"),
    (r"\'</sty\'le>", "'</sty'le>"),
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

const STYLE_QUOTED_CASES: [(&str, &str); 5] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"div::after { content: \'<style><\/stYle >\';}", "div::after { content: '<style></stYle >';}"),
    (
        "div::after { content: \\\"<style><\\/style>\\\";} label::after { content: \\'<style><\\/style >\\';}",
        "div::after { content: \"<style></style>\";} label::after { content: '<style></style >';}",
    ),
    (r"<\/style>1\'2\'3", "</style>1'2'3"),
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
