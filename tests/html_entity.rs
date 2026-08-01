const TEXT_MINIMAL_CASES: [(&str, &str); 7] = [
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
    for (expect, text) in TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_text_minimal(text));
    }
}

#[test]
fn encode_text_minimal_to_string() {
    for (expect, text) in TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_text_minimal_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_text_minimal_to_writer() {
    for (expect, text) in TEXT_MINIMAL_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_text_minimal_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_text_minimal() {
    for (text, expect) in TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_text_minimal_to_string() {
    for (text, expect) in TEXT_MINIMAL_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_text_minimal_to_writer() {
    for (text, expect) in TEXT_MINIMAL_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const SAFE_CASES: [(&str, &str); 7] = [
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
    for (expect, text) in SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_safe(text));
    }
}

#[test]
fn encode_safe_to_string() {
    for (expect, text) in SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_safe_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_safe_to_writer() {
    for (expect, text) in SAFE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_safe_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_safe() {
    for (text, expect) in SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_safe_to_string() {
    for (text, expect) in SAFE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_safe_to_writer() {
    for (text, expect) in SAFE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const UNQUOTED_ATTRIBUTE_CASES: [(&str, &str); 7] = [
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
    for (expect, text) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_unquoted_attribute(text));
    }
}

#[test]
fn encode_unquoted_attribute_to_string() {
    for (expect, text) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(
            expect,
            html_escape::encode_unquoted_attribute_to_string(text, &mut String::new())
        );
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_unquoted_attribute_to_writer() {
    for (expect, text) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::encode_unquoted_attribute_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

#[test]
fn decode_unquoted_attribute() {
    for (text, expect) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_unquoted_attribute_to_string() {
    for (text, expect) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_unquoted_attribute_to_writer() {
    for (text, expect) in UNQUOTED_ATTRIBUTE_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

// TODO ----------

const BIG_NAMES_COMPATIBILITY_CASES: [(&str, &str); 4] =
    [("&", "&AMP;"), ("<", "&LT;"), (">", "&GT;"), ("\"", "&QUOT;")];

#[test]
fn decode_html_entities() {
    for (expect, text) in BIG_NAMES_COMPATIBILITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_html_entities_to_string() {
    for (expect, text) in BIG_NAMES_COMPATIBILITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_to_writer() {
    for (expect, text) in BIG_NAMES_COMPATIBILITY_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const VALID_ENTITY_AFTER_MALFORMED_CASES: [(&str, &str); 6] = [
    ("&&amp;", "&&"),
    ("x &&amp; y", "x && y"),
    ("&bogus&amp;", "&bogus&"),
    ("&bogus &amp;", "&bogus &"),
    ("&#xZZ&amp;", "&#xZZ&"),
    ("&#bad&amp;", "&#bad&"),
];

#[test]
fn decode_html_entities_decodes_valid_entity_after_malformed() {
    for (text, expect) in VALID_ENTITY_AFTER_MALFORMED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_html_entities_decodes_valid_entity_after_malformed_to_string() {
    for (text, expect) in VALID_ENTITY_AFTER_MALFORMED_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_decodes_valid_entity_after_malformed_to_writer() {
    for (text, expect) in VALID_ENTITY_AFTER_MALFORMED_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const STRICT_ENTITY_CASES: [(&str, &str); 9] = [
    ("&amp", "&amp"),
    ("&notit;", "&notit;"),
    ("&#xD800;", "&#xD800;"),
    ("&#x110000;", "&#x110000;"),
    ("&#+65;", "&#+65;"),
    ("&#x+41;", "&#x+41;"),
    ("&#0;", "&#0;"),
    ("&#1;", "&#1;"),
    ("&#xB;", "&#xB;"),
];

#[test]
fn decode_html_entities_keeps_strict_cases() {
    for (text, expect) in STRICT_ENTITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_html_entities_keeps_strict_cases_to_string() {
    for (text, expect) in STRICT_ENTITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_keeps_strict_cases_to_writer() {
    for (text, expect) in STRICT_ENTITY_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const ASCII_WHITESPACE_CASES: [(&str, &str); 4] =
    [("&#9;", "\t"), ("&#10;", "\n"), ("&#12;", "\u{000C}"), ("&#xD;", "\r")];

#[test]
fn decode_html_entities_decodes_ascii_whitespace() {
    for (text, expect) in ASCII_WHITESPACE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}
