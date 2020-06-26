extern crate html_escape;

const ENCODE_TEXT_MINIMAL_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("麵包 &amp; butter", "麵包 & butter"),
    ("\"bread\" &amp; 奶油", "\"bread\" & 奶油"),
    ("&lt; less than", "< less than"),
    ("greater than >", "greater than >"),
    ("https://magiclen.org", "https://magiclen.org"),
];

#[test]
fn encode_text_minimal() {
    for (expect, actual) in ENCODE_TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_text_minimal(actual));
    }
}

#[test]
fn encode_text_minimal_to_string() {
    for (expect, actual) in ENCODE_TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_text_minimal_to_string(actual, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_text_minimal_to_writer() {
    for (expect, actual) in ENCODE_TEXT_MINIMAL_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_text_minimal_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const ENCODE_SAFE_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("麵包 &amp; butter", "麵包 & butter"),
    ("&quot;bread&quot; &amp; 奶油", "\"bread\" & 奶油"),
    ("&lt; less than", "< less than"),
    ("greater than &gt;", "greater than >"),
    ("https:&#x2F;&#x2F;magiclen.org", "https://magiclen.org"),
];

#[test]
fn encode_safe() {
    for (expect, actual) in ENCODE_SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_safe(actual));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_safe_to_string() {
    for (expect, actual) in ENCODE_SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_safe_to_string(actual, &mut String::new()));
    }
}

#[test]
fn encode_safe_to_writer() {
    for (expect, actual) in ENCODE_SAFE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_safe_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const ENCODE_UNQUOTED_ATTRIBUTE_CASES: [(&str, &str); 7] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("&quot;bread&quot;&#x20;&amp;&#x20;奶油", "\"bread\" & 奶油"),
    ("&lt;&#x20;less&#x20;than", "< less than"),
    ("greater&#x20;than&#x20;&gt;", "greater than >"),
    ("https&#x3A;&#x2F;&#x2F;magiclen&#x2E;org", "https://magiclen.org"),
    ("d&#x2D;none&#x20;m&#x2D;0", "d-none m-0"),
];

#[test]
fn encode_unquoted_attribute() {
    for (expect, actual) in ENCODE_UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_unquoted_attribute(actual));
    }
}

#[test]
fn encode_unquoted_attribute_to_string() {
    for (expect, actual) in ENCODE_UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_unquoted_attribute_to_string(actual, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_unquoted_attribute_to_writer() {
    for (expect, actual) in ENCODE_UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_unquoted_attribute_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const ENCODE_SCRIPT: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"alert('<script><\/script>');", "alert('<script></script>');"),
    (
        r"alert('<script><\/script>');alert('<script><\/script>');",
        "alert('<script></script>');alert('<script></script>');",
    ),
];

#[test]
fn encode_script() {
    for (expect, actual) in ENCODE_SCRIPT.iter().copied() {
        assert_eq!(expect, html_escape::encode_script(actual));
    }
}

#[test]
fn encode_script_to_string() {
    for (expect, actual) in ENCODE_SCRIPT.iter().copied() {
        assert_eq!(expect, html_escape::encode_script_to_string(actual, &mut String::new()));
    }
}

#[test]
fn encode_script_to_writer() {
    for (expect, actual) in ENCODE_SCRIPT.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_script_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const ENCODE_STYLE: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    (r"div::after { content: '<style><\/style>';}", "div::after { content: '<style></style>';}"),
    (
        r"div::after { content: '<style><\/style>';} label::after { content: '<style><\/style>';}",
        "div::after { content: '<style></style>';} label::after { content: '<style></style>';}",
    ),
];

#[test]
fn encode_style() {
    for (expect, actual) in ENCODE_STYLE.iter().copied() {
        assert_eq!(expect, html_escape::encode_style(actual));
    }
}

#[test]
fn encode_style_to_string() {
    for (expect, actual) in ENCODE_STYLE.iter().copied() {
        assert_eq!(expect, html_escape::encode_style_to_string(actual, &mut String::new()));
    }
}

#[test]
fn encode_style_to_writer() {
    for (expect, actual) in ENCODE_STYLE.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_style_to_writer(actual, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}
