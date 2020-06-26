extern crate html_escape;

const DECODE_CASES: [(&str, &str); 11] = [
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
fn decode() {
    for (expect, actual) in DECODE_CASES.iter().copied() {
        assert_eq!(expect, html_escape::decode(actual));
    }
}
