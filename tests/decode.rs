extern crate html_escape;

const DECODE_HTML_ENTITIES_CASES: [(&str, &str); 11] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("&", "&AMP;"),
    ("<", "&LT;"),
    (">", "&GT;"),
    ("\"", "&QUOT;"),
    ("\"bread\" & 奶油", "&quot;bread&quot;&#x20;&amp;&#x20;奶油"),
    ("< less than", "&lt;&#x20;less&#x20;than"),
    ("greater than >", "greater&#x20;than&#x20;&gt;"),
    ("https://magiclen.org", "https&#x3A;&#x2F;&#x2F;magiclen&#x2E;org"),
    ("d-none m-0", "d&#x2D;none&#x20;m&#x2D;0"),
];

#[test]
fn decode_html_entities() {
    for (expect, actual) in DECODE_HTML_ENTITIES_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(actual));
    }
}

#[test]
fn decode_html_entities_to_string() {
    for (expect, actual) in DECODE_HTML_ENTITIES_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(actual, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_to_writer() {
    for (expect, actual) in DECODE_HTML_ENTITIES_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const DECODE_SCRIPT_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("alert('<script></scrIpt >');", r"alert('<script><\/scrIpt >');"),
    (
        "alert('<script></script>');alert('<script></script >');",
        r"alert('<script><\/script>');alert('<script><\/script >');",
    ),
];

#[test]
fn decode_script() {
    for (expect, actual) in DECODE_SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script(actual));
    }
}

#[test]
fn decode_script_to_string() {
    for (expect, actual) in DECODE_SCRIPT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_script_to_string(actual, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_script_to_writer() {
    for (expect, actual) in DECODE_SCRIPT_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_script_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const DECODE_STYLE_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("div::after { content: '<style></stYle >';}", r"div::after { content: '<style><\/stYle >';}"),
    (
        "div::after { content: '<style></style>';} label::after { content: '<style></style >';}",
        r"div::after { content: '<style><\/style>';} label::after { content: '<style><\/style >';}",
    ),
];

#[test]
fn decode_style() {
    for (expect, actual) in DECODE_STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style(actual));
    }
}

#[test]
fn decode_style_to_string() {
    for (expect, actual) in DECODE_STYLE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_style_to_string(actual, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_style_to_writer() {
    for (expect, actual) in DECODE_STYLE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_style_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}
