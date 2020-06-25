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

fn encode_attribute(bencher: &mut Bencher) {
    let text = fs::read_to_string(TEXT_PATH).unwrap();
    let length = text.len();

    bencher.iter(|| html_escape::encode_unquoted_attribute(text.as_str()));
    bencher.bytes = length as u64;
}

benchmark_group!(group, encode_attribute,);

benchmark_main!(group);
