use core::str::from_utf8_unchecked;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

macro_rules! parse_script {
    ($e:expr, $step:ident, $b:block, $bq:block, $($addi:expr),+) => {
        match $step {
            0 => {
                match $e {
                    b'<' => $step = 1,
                    b'\\' => $step = 10,
                    $(| $addi)+ => $bq,
                    _ => (),
                }
            }
            1 => {
                match $e {
                    b'/' => $step = 2,
                    _ => $step = 0,
                }
            }
            2 => {
                match $e {
                    b's' | b'S' => $step = 3,
                    _ => $step = 0,
                }
            }
            3 => {
                match $e {
                    b'c' | b'C' => $step = 4,
                    _ => $step = 0,
                }
            }
            4 => {
                match $e {
                    b'r' | b'R' => $step = 5,
                    _ => $step = 0,
                }
            }
            5 => {
                match $e {
                    b'i' | b'I' => $step = 6,
                    _ => $step = 0,
                }
            }
            6 => {
                match $e {
                    b'p' | b'P' => $step = 7,
                    _ => $step = 0,
                }
            }
            7 => {
                match $e {
                    b't' | b'T' => $step = 8,
                    _ => $step = 0,
                }
            }
            8 => {
                $step = 0;

                match $e {
                    b' ' | b'>' => $b
                    _ => (),
                }
            }
            10 => {
                match $e {
                    b'<' => $step = 1,
                    _ => $step = 0,
                }
            }
            _ => unreachable!(),
        }
    };
}

macro_rules! parse_text_single_quoted {
    ($e:expr, $step:ident, $b:block, $bq:block) => {
        parse_script!($e, $step, $b, $bq, b'\'');
    };
}

macro_rules! parse_text_double_quoted {
    ($e:expr, $step:ident, $b:block, $bq:block) => {
        parse_script!($e, $step, $b, $bq, b'"');
    };
}

macro_rules! parse_text_quoted {
    ($e:expr, $step:ident, $b:block, $bq:block) => {
        parse_script!($e, $step, $b, $bq, b'\'', b'"');
    };
}

pub fn encode_javascript_single_quoted_text<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    let mut step = 0;

    let (mut v, mut start) = loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        parse_text_single_quoted!(
            e,
            step,
            {
                let mut v = Vec::with_capacity(text_length + 1);

                v.extend_from_slice(&text_bytes[..(p - 7)]);

                break (v, p - 7);
            },
            {
                let mut v = Vec::with_capacity(text_length + 1);

                v.extend_from_slice(&text_bytes[..p]);

                break (v, p);
            }
        );

        p += 1;
    };

    v.push(b'\\');

    p += 1;

    for e in text_bytes[p..].iter().copied() {
        parse_text_single_quoted!(
            e,
            step,
            {
                v.extend_from_slice(&text_bytes[start..(p - 7)]);
                start = p - 7;
                v.push(b'\\');
            },
            {
                v.extend_from_slice(&text_bytes[start..p]);
                start = p;
                v.push(b'\\');
            }
        );

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

#[inline]
pub fn encode_javascript_single_quoted_text_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(encode_javascript_single_quoted_text_to_vec(text, output.as_mut_vec())) }
}

pub fn encode_javascript_single_quoted_text_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_text_single_quoted!(
            e,
            step,
            {
                output.extend_from_slice(&text_bytes[start..(end - 7)]);
                start = end - 7;
                output.push(b'\\');
            },
            {
                output.extend_from_slice(&text_bytes[start..end]);
                start = end;
                output.push(b'\\');
            }
        );

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
pub fn encode_javascript_single_quoted_text_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_text_single_quoted!(
            e,
            step,
            {
                output.write_all(&text_bytes[start..(end - 7)])?;
                start = end - 7;
                output.write_all(b"\\")?;
            },
            {
                output.write_all(&text_bytes[start..end])?;
                start = end;
                output.write_all(b"\\")?;
            }
        );

        end += 1;
    }

    output.write_all(&text_bytes[start..end])
}