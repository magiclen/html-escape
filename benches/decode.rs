use bencher::{benchmark_group, benchmark_main, black_box, Bencher};

macro_rules! decode_benchmarks {
    ($group:ident, $module:ident, $decode:ident, $to_string:ident, $text:expr) => {
        mod $module {
            use super::*;

            pub fn cow(bencher: &mut Bencher) {
                let text = $text.repeat(64);
                bencher.bytes = text.len() as u64;
                bencher.iter(|| html_escape::$decode(black_box(text.as_str())));
            }

            pub fn fresh_string(bencher: &mut Bencher) {
                let text = $text.repeat(64);
                bencher.bytes = text.len() as u64;
                bencher.iter(|| {
                    let mut output = String::new();
                    html_escape::$to_string(black_box(text.as_str()), &mut output);
                    output
                });
            }

            pub fn reused_string(bencher: &mut Bencher) {
                let text = $text.repeat(64);
                let mut output = html_escape::$decode(text.as_str()).into_owned();
                bencher.bytes = text.len() as u64;
                bencher.iter(|| {
                    output.clear();
                    black_box(html_escape::$to_string(black_box(text.as_str()), &mut output));
                });
            }
        }

        benchmark_group!($group, $module::cow, $module::fresh_string, $module::reused_string);
    };
}

decode_benchmarks!(
    no_entities,
    no_entity_cases,
    decode_html_entities,
    decode_html_entities_to_string,
    "ordinary text 中文🙂 0123456789 "
);
decode_benchmarks!(
    named_entities,
    named_entity_cases,
    decode_html_entities,
    decode_html_entities_to_string,
    "中文&amp;&lt;&gt;&quot;&apos;"
);
decode_benchmarks!(
    numeric_entities,
    numeric_entity_cases,
    decode_html_entities,
    decode_html_entities_to_string,
    "&#65;&#x20;&#20013;&#x6587;&#x1F642;"
);
decode_benchmarks!(
    multi_codepoint_entities,
    multi_codepoint_cases,
    decode_html_entities,
    decode_html_entities_to_string,
    "中文&fjlig;&NotEqualTilde;&ThickSpace;&nGt;"
);
decode_benchmarks!(
    script,
    script_cases,
    decode_script,
    decode_script_to_string,
    r"<\/script><\!--"
);
decode_benchmarks!(style, style_cases, decode_style, decode_style_to_string, r"<\/style><\!--");
decode_benchmarks!(
    script_quoted,
    script_quoted_cases,
    decode_script_quoted_text,
    decode_script_quoted_text_to_string,
    r#"中文\x3C/script>\"\'\\\x0A\u2028"#
);
decode_benchmarks!(
    style_quoted,
    style_quoted_cases,
    decode_style_quoted_text,
    decode_style_quoted_text_to_string,
    r#"中文\3C /style>\"\'\\\0A "#
);

benchmark_main!(
    no_entities,
    named_entities,
    numeric_entities,
    multi_codepoint_entities,
    script,
    style,
    script_quoted,
    style_quoted,
);
