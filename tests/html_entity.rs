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

const TEXT_CASES: [(&str, &str); 5] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("\"bread\" &amp; 奶油", "\"bread\" & 奶油"),
    ("&lt; less than", "< less than"),
    ("greater than &gt;", "greater than >"),
];

#[test]
fn encode_text() {
    for (expect, text) in TEXT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_text(text));
    }
}

const DOUBLE_QUOTED_ATTRIBUTE_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("&quot;bread&quot; &amp; 奶油", "\"bread\" & 奶油"),
    ("'single' &lt;tag&gt;", "'single' <tag>"),
];

#[test]
fn encode_double_quoted_attribute() {
    for (expect, text) in DOUBLE_QUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_double_quoted_attribute(text));
    }
}

const SINGLE_QUOTED_ATTRIBUTE_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("&#x27;bread&#x27; &amp; 奶油", "'bread' & 奶油"),
    ("\"double\" &lt;tag&gt;", "\"double\" <tag>"),
];

#[test]
fn encode_single_quoted_attribute() {
    for (expect, text) in SINGLE_QUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_single_quoted_attribute(text));
    }
}

const QUOTED_ATTRIBUTE_CASES: [(&str, &str); 4] = [
    ("", ""),
    ("哈囉，中文！", "哈囉，中文！"),
    ("&quot;bread&quot; &amp; &#x27;奶油&#x27;", "\"bread\" & '奶油'"),
    ("&lt;tag&gt;", "<tag>"),
];

#[test]
fn encode_quoted_attribute() {
    for (expect, text) in QUOTED_ATTRIBUTE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::encode_quoted_attribute(text));
    }
}

#[test]
fn html_entity_to_vec_appends_and_returns_written_data() {
    let mut encoded = b"prefix:".to_vec();

    assert_eq!(
        b"&quot;bread&quot; &amp; &#x27;butter&#x27;",
        html_escape::encode_quoted_attribute_to_vec("\"bread\" & 'butter'", &mut encoded)
    );
    assert_eq!(b"prefix:&quot;bread&quot; &amp; &#x27;butter&#x27;", encoded.as_slice());

    let mut decoded = b"prefix:".to_vec();

    assert_eq!(
        b"<bread & butter>",
        html_escape::decode_html_entities_to_vec("&lt;bread &amp; butter&gt;", &mut decoded)
    );
    assert_eq!(b"prefix:<bread & butter>", decoded.as_slice());
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

const UNICODE_NUMERIC_ENTITY_CASES: [(&str, &str); 5] = [
    ("中", "&#20013;"),
    ("文", "&#x6587;"),
    ("🙂", "&#128578;"),
    ("🙂", "&#X1F642;"),
    ("中文A文🙂結尾", "中文&#65;&#x6587;&#128578;結尾"),
];

#[test]
fn decode_html_entities_decodes_unicode_numeric_entities() {
    for (expect, text) in UNICODE_NUMERIC_ENTITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_html_entities_decodes_unicode_numeric_entities_to_string() {
    for (expect, text) in UNICODE_NUMERIC_ENTITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_decodes_unicode_numeric_entities_to_writer() {
    for (expect, text) in UNICODE_NUMERIC_ENTITY_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const MULTI_CODEPOINT_CASES: [(&str, &str); 5] = [
    ("fj", "&fjlig;"),
    ("\u{2242}\u{0338}", "&NotEqualTilde;"),
    ("\u{205F}\u{200A}", "&ThickSpace;"),
    ("\u{226B}\u{20D2}", "&nGt;"),
    (
        "中文fj\u{2242}\u{0338}\u{205F}\u{200A}\u{226B}\u{20D2}結尾",
        "中文&fjlig;&NotEqualTilde;&ThickSpace;&nGt;結尾",
    ),
];

#[test]
fn decode_html_entities_decodes_multi_codepoint_entities() {
    for (expect, text) in MULTI_CODEPOINT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[test]
fn decode_html_entities_decodes_multi_codepoint_entities_to_string() {
    for (expect, text) in MULTI_CODEPOINT_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities_to_string(text, &mut String::new()));
    }
}

#[cfg(feature = "std")]
#[test]
fn decode_html_entities_decodes_multi_codepoint_entities_to_writer() {
    for (expect, text) in MULTI_CODEPOINT_CASES.iter().copied() {
        let mut v = Vec::new();
        html_escape::decode_html_entities_to_writer(text, &mut v).unwrap();

        assert_eq!(expect.as_bytes(), v.as_slice());
    }
}

const COMMON_NAMED_ENTITY_CASES: [(&str, &str); 2] = [("'", "&apos;"), ("\u{00A0}", "&nbsp;")];

#[test]
fn decode_html_entities_decodes_common_named_entities() {
    for (expect, text) in COMMON_NAMED_ENTITY_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode_html_entities(text));
    }
}

#[cfg(feature = "std")]
#[test]
fn encode_html_entities_to_dyn_writer() {
    let mut bytes = b"prefix:".to_vec();
    let writer: &mut dyn std::io::Write = &mut bytes;

    html_escape::encode_text_to_writer("中文 & <tag>", writer).unwrap();
    html_escape::encode_unquoted_attribute_to_writer(" a/b", writer).unwrap();

    assert_eq!("prefix:中文 &amp; &lt;tag&gt;&#x20;a&#x2F;b".as_bytes(), bytes.as_slice());
}
