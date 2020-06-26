/*!
This benchmark is referred to https://github.com/veddan/rust-htmlescape/blob/master/benches/bench.rs.
*/

extern crate html_escape;

#[macro_use]
extern crate bencher;

use std::fs;

use bencher::Bencher;

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

benchmark_main!(
    text_minimal,
    text,
    double_quoted_attribute,
    single_quoted_attribute,
    quoted_attribute,
    unquoted_attribute,
    safe
);
