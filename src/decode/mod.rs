mod named_entities;

use core::convert::TryFrom;
use core::str::from_utf8_unchecked;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::functions::*;

pub use named_entities::*;

/// Decode html entities in a given string.
pub fn decode_html_entities<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut ep = 0;
    let mut e;

    let mut step = 0;

    let (mut v, mut start) = loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = p;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let name = &text_bytes[(ep + 1)..p];

                    match NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name)) {
                        Ok(index) => {
                            v.extend_from_slice(NAMED_ENTITIES[index].1.as_bytes());
                            break (v, p + 1);
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let number = unsafe { text.get_unchecked((ep + 2)..p) };

                    match number.parse::<u32>() {
                        Ok(number) => {
                            match char::try_from(number) {
                                Ok(c) => {
                                    write_char_to_vec(c, &mut v);
                                    break (v, p + 1);
                                }
                                Err(_) => break (v, ep),
                            }
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let hex = unsafe { text.get_unchecked((ep + 3)..p) };

                    match u32::from_str_radix(hex, 16) {
                        Ok(number) => {
                            match char::try_from(number) {
                                Ok(c) => {
                                    write_char_to_vec(c, &mut v);
                                    break (v, p + 1);
                                }
                                Err(_) => break (v, ep),
                            }
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            _ => unreachable!(),
        }

        p += 1;
    };

    p += 1;

    step = 0;

    for e in text_bytes[p..].iter().copied() {
        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = p;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    step = 0;

                    let name = &text_bytes[(ep + 1)..p];

                    if let Ok(index) =
                        NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name))
                    {
                        v.extend_from_slice(&text_bytes[start..ep]);
                        start = p + 1;
                        v.extend_from_slice(NAMED_ENTITIES[index].1.as_bytes());
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    step = 0;

                    let number = unsafe { text.get_unchecked((ep + 2)..p) };

                    if let Ok(number) = number.parse::<u32>() {
                        if let Ok(c) = char::try_from(number) {
                            v.extend_from_slice(&text_bytes[start..ep]);
                            start = p + 1;
                            write_char_to_vec(c, &mut v);
                        }
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    step = 0;

                    let hex = unsafe { text.get_unchecked((ep + 3)..p) };

                    if let Ok(number) = u32::from_str_radix(hex, 16) {
                        if let Ok(c) = char::try_from(number) {
                            v.extend_from_slice(&text_bytes[start..ep]);
                            start = p + 1;
                            write_char_to_vec(c, &mut v);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Decode html entities in a given string to a mutable `String` reference and return the decoded string slice.
pub fn decode_html_entities_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(decode_html_entities_to_vec(text, output.as_mut_vec())) }
}

/// Decode html entities in a given string to a mutable `Vec<u8>` reference and return the decoded data slice.
pub fn decode_html_entities_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut start = 0;
    let mut end = 0;
    let mut ep = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = end;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        output.extend_from_slice(&text_bytes[start..=end]);
                        start = end + 1;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    step = 0;

                    let name = &text_bytes[(ep + 1)..end];

                    if let Ok(index) =
                        NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name))
                    {
                        output.extend_from_slice(&text_bytes[start..ep]);
                        start = end + 1;
                        output.extend_from_slice(NAMED_ENTITIES[index].1.as_bytes());
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        output.extend_from_slice(&text_bytes[start..=end]);
                        start = end + 1;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    step = 0;

                    let number = unsafe { text.get_unchecked((ep + 2)..end) };

                    if let Ok(number) = number.parse::<u32>() {
                        if let Ok(c) = char::try_from(number) {
                            output.extend_from_slice(&text_bytes[start..ep]);
                            start = end + 1;
                            write_char_to_vec(c, output);
                        }
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;

                        output.extend_from_slice(&text_bytes[start..=end]);
                        start = end + 1;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    step = 0;

                    let hex = unsafe { text.get_unchecked((ep + 3)..end) };

                    if let Ok(number) = u32::from_str_radix(hex, 16) {
                        if let Ok(c) = char::try_from(number) {
                            output.extend_from_slice(&text_bytes[start..ep]);
                            start = end + 1;
                            write_char_to_vec(c, output);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Decode html entities in a given string to a writer.
pub fn decode_html_entities_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let mut start = 0;
    let mut end = 0;
    let mut ep = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = end;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        output.write_all(&text_bytes[start..=end])?;
                        start = end + 1;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    step = 0;

                    let name = &text_bytes[(ep + 1)..end];

                    if let Ok(index) =
                        NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name))
                    {
                        output.write_all(&text_bytes[start..ep])?;
                        start = end + 1;
                        output.write_all(NAMED_ENTITIES[index].1.as_bytes())?;
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        output.write_all(&text_bytes[start..=end])?;
                        start = end + 1;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    step = 0;

                    let number = unsafe { text.get_unchecked((ep + 2)..end) };

                    if let Ok(number) = number.parse::<u32>() {
                        if let Ok(c) = char::try_from(number) {
                            output.write_all(&text_bytes[start..ep])?;
                            start = end + 1;
                            write_char_to_writer(c, output)?;
                        }
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;

                        output.write_all(&text_bytes[start..=end])?;
                        start = end + 1;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    step = 0;

                    let hex = unsafe { text.get_unchecked((ep + 3)..end) };

                    if let Ok(number) = u32::from_str_radix(hex, 16) {
                        if let Ok(c) = char::try_from(number) {
                            output.write_all(&text_bytes[start..ep])?;
                            start = end + 1;
                            write_char_to_writer(c, output)?;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        end += 1;
    }

    output.write_all(&text_bytes[start..end])
}

// TODO ----------

macro_rules! parse_script {
    ($e:expr, $step:ident, $b:block) => {
        match $step {
            0 => {
                match $e {
                    b'<' => $step = 1,
                    _ => (),
                }
            }
            1 => {
                match $e {
                    b'\\' => $step = 2,
                    _ => (),
                }
            }
            2 => {
                match $e {
                    b'/' => $step = 3,
                    _ => $step = 0,
                }
            }
            3 => {
                match $e {
                    b's' | b'S' => $step = 4,
                    _ => $step = 0,
                }
            }
            4 => {
                match $e {
                    b'c' | b'C' => $step = 5,
                    _ => $step = 0,
                }
            }
            5 => {
                match $e {
                    b'r' | b'R' => $step = 6,
                    _ => $step = 0,
                }
            }
            6 => {
                match $e {
                    b'i' | b'I' => $step = 7,
                    _ => $step = 0,
                }
            }
            7 => {
                match $e {
                    b'p' | b'P' => $step = 8,
                    _ => $step = 0,
                }
            }
            8 => {
                match $e {
                    b't' | b'T' => $step = 9,
                    _ => $step = 0,
                }
            }
            9 => {
                $step = 0;

                match $e {
                    b' ' | b'>' => $b
                    _ => (),
                }
            }
            _ => unreachable!(),
        }
    };
    ($e:expr, $step:ident) => {
        parse_script!($e, $step, { break; });
    };
}

/// Decode script text in a given string. Unescape `<\/script>` to `</script>`.
pub fn decode_script<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    let mut step = 0;

    loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        parse_script!(e, step);

        p += 1;
    }

    let mut v = Vec::with_capacity(text_length);

    v.extend_from_slice(&text_bytes[..(p - 8)]);

    let mut start = p - 7;

    p += 1;

    for e in text_bytes[p..].iter().copied() {
        parse_script!(e, step, {
            v.extend_from_slice(&text_bytes[start..(p - 8)]);
            start = p + 1;
            v.extend_from_slice(&text_bytes[(p - 7)..=p]);
        });

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Decode script text in a given string to a mutable `String` reference and return the decoded string slice. Unescape `<\/script>` to `</script>`.
#[inline]
pub fn decode_script_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(decode_script_to_vec(text, output.as_mut_vec())) }
}

/// Decode script text in a given string to a mutable `Vec<u8>` reference and return the decoded data slice. Unescape `<\/script>` to `</script>`.
pub fn decode_script_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_script!(e, step, {
            output.extend_from_slice(&text_bytes[start..(end - 8)]);
            start = end + 1;
            output.extend_from_slice(&text_bytes[(end - 7)..=end]);
        });

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Decode script text in a given string to a writer. Unescape `<\/script>` to `</script>`.
pub fn decode_script_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_script!(e, step, {
            output.write_all(&text_bytes[start..(end - 8)])?;
            start = end + 1;
            output.write_all(&text_bytes[(end - 7)..=end])?;
        });

        end += 1;
    }

    output.write_all(&text_bytes[start..end])
}

// TODO ----------

macro_rules! parse_style {
    ($e:expr, $step:ident, $b:block) => {
        match $step {
            0 => {
                match $e {
                    b'<' => $step = 1,
                    _ => (),
                }
            }
            1 => {
                match $e {
                    b'\\' => $step = 2,
                    _ => (),
                }
            }
            2 => {
                match $e {
                    b'/' => $step = 3,
                    _ => $step = 0,
                }
            }
            3 => {
                match $e {
                    b's' | b'S' => $step = 4,
                    _ => $step = 0,
                }
            }
            4 => {
                match $e {
                    b't' | b'T' => $step = 5,
                    _ => $step = 0,
                }
            }
            5 => {
                match $e {
                    b'y' | b'Y' => $step = 6,
                    _ => $step = 0,
                }
            }
            6 => {
                match $e {
                    b'l' | b'L' => $step = 7,
                    _ => $step = 0,
                }
            }
            7 => {
                match $e {
                    b'e' | b'E' => $step = 8,
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
            _ => unreachable!(),
        }
    };
    ($e:expr, $step:ident) => {
        parse_style!($e, $step, { break; });
    };
}

/// Decode style text in a given string. Unescape `<\/style>` to `</style>`.
pub fn decode_style<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    let mut step = 0;

    loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        parse_style!(e, step);

        p += 1;
    }

    let mut v = Vec::with_capacity(text_length);

    v.extend_from_slice(&text_bytes[..(p - 7)]);

    let mut start = p - 6;

    p += 1;

    for e in text_bytes[p..].iter().copied() {
        parse_style!(e, step, {
            v.extend_from_slice(&text_bytes[start..(p - 7)]);
            start = p + 1;
            v.extend_from_slice(&text_bytes[(p - 6)..=p]);
        });

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Decode style text in a given string to a mutable `String` reference and return the decoded string slice. Unescape `<\/style>` to `</style>`.
#[inline]
pub fn decode_style_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(decode_style_to_vec(text, output.as_mut_vec())) }
}

/// Decode style text in a given string to a mutable `Vec<u8>` reference and return the decoded data slice. Unescape `<\/style>` to `</style>`.
pub fn decode_style_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_style!(e, step, {
            output.extend_from_slice(&text_bytes[start..(end - 7)]);
            start = end + 1;
            output.extend_from_slice(&text_bytes[(end - 6)..=end]);
        });

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Decode style text in a given string to a writer. Unescape `<\/style>` to `</style>`.
pub fn decode_style_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let mut start = 0;
    let mut end = 0;

    let mut step = 0;

    for e in text_bytes.iter().copied() {
        parse_style!(e, step, {
            output.write_all(&text_bytes[start..(end - 7)])?;
            start = end + 1;
            output.write_all(&text_bytes[(end - 6)..=end])?;
        });

        end += 1;
    }

    output.write_all(&text_bytes[start..end])
}
