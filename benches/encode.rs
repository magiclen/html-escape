use std::fs;

use bencher::{benchmark_group, benchmark_main, black_box, Bencher};

#[cfg(unix)]
const TEXT_PATH: &str = "benches/data/vgilante.txt";

#[cfg(windows)]
const TEXT_PATH: &str = r"benches\data\vgilante.txt";

macro_rules! benchmark_impl {
    ($g:ident, $f:ident, $f_to_string:ident, $f_to_writer:ident) => {
        fn $f(bencher: &mut Bencher) {
            let text = fs::read_to_string(TEXT_PATH).unwrap();
            let length = text.len();

            bencher.iter(|| html_escape::$f(text.as_str()));
            bencher.bytes = length as u64;
        }

        fn $f_to_string(bencher: &mut Bencher) {
            let text = fs::read_to_string(TEXT_PATH).unwrap();
            let length = text.len();

            bencher.iter(|| {
                let mut s = String::new();

                html_escape::$f_to_string(text.as_str(), &mut s);

                s
            });
            bencher.bytes = length as u64;
        }

        fn $f_to_writer(bencher: &mut Bencher) {
            let text = fs::read_to_string(TEXT_PATH).unwrap();
            let length = text.len();

            bencher.iter(|| {
                let mut v = Vec::new();

                html_escape::$f_to_writer(text.as_str(), &mut v).unwrap();

                v
            });
            bencher.bytes = length as u64;
        }

        benchmark_group!($g, $f, $f_to_string, $f_to_writer);
    };
}

benchmark_impl!(
    text_minimal,
    encode_text_minimal,
    encode_text_minimal_to_string,
    encode_text_minimal_to_writer
);

benchmark_impl!(text, encode_text, encode_text_to_string, encode_text_to_writer);

benchmark_impl!(
    double_quoted_attribute,
    encode_double_quoted_attribute,
    encode_double_quoted_attribute_to_string,
    encode_double_quoted_attribute_to_writer
);

benchmark_impl!(
    single_quoted_attribute,
    encode_single_quoted_attribute,
    encode_single_quoted_attribute_to_string,
    encode_single_quoted_attribute_to_writer
);

benchmark_impl!(
    quoted_attribute,
    encode_quoted_attribute,
    encode_quoted_attribute_to_string,
    encode_quoted_attribute_to_writer
);

benchmark_impl!(
    unquoted_attribute,
    encode_unquoted_attribute,
    encode_unquoted_attribute_to_string,
    encode_unquoted_attribute_to_writer
);

benchmark_impl!(safe, encode_safe, encode_safe_to_string, encode_safe_to_writer);

benchmark_impl!(script, encode_script, encode_script_to_string, encode_script_to_writer);
benchmark_impl!(
    script_single_quoted_text,
    encode_script_single_quoted_text,
    encode_script_single_quoted_text_to_string,
    encode_script_single_quoted_text_to_writer
);
benchmark_impl!(
    script_double_quoted_text,
    encode_script_double_quoted_text,
    encode_script_double_quoted_text_to_string,
    encode_script_double_quoted_text_to_writer
);
benchmark_impl!(
    script_quoted_text,
    encode_script_quoted_text,
    encode_script_quoted_text_to_string,
    encode_script_quoted_text_to_writer
);

benchmark_impl!(style, encode_style, encode_style_to_string, encode_style_to_writer);
benchmark_impl!(
    style_single_quoted_text,
    encode_style_single_quoted_text,
    encode_style_single_quoted_text_to_string,
    encode_style_single_quoted_text_to_writer
);
benchmark_impl!(
    style_double_quoted_text,
    encode_style_double_quoted_text,
    encode_style_double_quoted_text_to_string,
    encode_style_double_quoted_text_to_writer
);
benchmark_impl!(
    style_quoted_text,
    encode_style_quoted_text,
    encode_style_quoted_text_to_string,
    encode_style_quoted_text_to_writer
);

macro_rules! scenario_benchmarks {
    ($group:ident, $module:ident, $encode:ident, $to_string:ident, $special:expr) => {
        mod $module {
            use super::*;

            const CLEAN: &str = "ordinary text 0123456789 ";

            fn measure(bencher: &mut Bencher, text: String) {
                bencher.bytes = text.len() as u64;
                bencher.iter(|| html_escape::$encode(black_box(text.as_str())));
            }

            pub fn clean(bencher: &mut Bencher) {
                measure(bencher, CLEAN.repeat(64));
            }

            pub fn sparse(bencher: &mut Bencher) {
                measure(bencher, format!("{}{}{}", CLEAN.repeat(32), $special, CLEAN.repeat(32)));
            }

            pub fn dense(bencher: &mut Bencher) {
                measure(bencher, $special.repeat(64));
            }

            pub fn unicode(bencher: &mut Bencher) {
                measure(bencher, format!("中文🙂{}臺灣", $special).repeat(64));
            }

            pub fn dense_fresh_string(bencher: &mut Bencher) {
                let text = $special.repeat(64);
                bencher.bytes = text.len() as u64;
                bencher.iter(|| {
                    let mut output = String::new();
                    html_escape::$to_string(black_box(text.as_str()), &mut output);
                    output
                });
            }

            pub fn dense_reused_string(bencher: &mut Bencher) {
                let text = $special.repeat(64);
                let mut output = html_escape::$encode(text.as_str()).into_owned();
                bencher.bytes = text.len() as u64;
                bencher.iter(|| {
                    output.clear();
                    black_box(html_escape::$to_string(black_box(text.as_str()), &mut output));
                });
            }
        }

        benchmark_group!(
            $group,
            $module::clean,
            $module::sparse,
            $module::dense,
            $module::unicode,
            $module::dense_fresh_string,
            $module::dense_reused_string
        );
    };
}

scenario_benchmarks!(
    text_scenarios,
    text_cases,
    encode_text,
    encode_text_to_string,
    "<tag>&text</tag>"
);
scenario_benchmarks!(
    script_scenarios,
    script_cases,
    encode_script,
    encode_script_to_string,
    "</script><!--"
);
scenario_benchmarks!(
    style_scenarios,
    style_cases,
    encode_style,
    encode_style_to_string,
    "</style><!--"
);
scenario_benchmarks!(
    script_quoted_scenarios,
    script_quoted_cases,
    encode_script_quoted_text,
    encode_script_quoted_text_to_string,
    "</script>\"'\\\n\u{2028}"
);
scenario_benchmarks!(
    style_quoted_scenarios,
    style_quoted_cases,
    encode_style_quoted_text,
    encode_style_quoted_text_to_string,
    "</style>\"'\\\n\0"
);

benchmark_main!(
    text_minimal,
    text,
    double_quoted_attribute,
    single_quoted_attribute,
    quoted_attribute,
    unquoted_attribute,
    safe,
    script,
    script_single_quoted_text,
    script_double_quoted_text,
    script_quoted_text,
    style,
    style_single_quoted_text,
    style_double_quoted_text,
    style_quoted_text,
    text_scenarios,
    script_scenarios,
    style_scenarios,
    script_quoted_scenarios,
    style_quoted_scenarios,
);
