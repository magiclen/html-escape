use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::functions::write_char_to_vec;
#[cfg(feature = "std")]
use crate::functions::write_char_to_writer;

#[inline]
fn hex_digit(e: u8) -> Option<u8> {
    match e {
        b'0'..=b'9' => Some(e - b'0'),
        b'A'..=b'F' => Some(e - b'A' + 10),
        _ => None,
    }
}

/// Decode the escape which starts with the backslash at `text[p]`.
///
/// Callers check the backslash first, so most bytes do not need to call this function.
#[inline]
fn decode_escape<const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &[u8],
    p: usize,
) -> Option<(usize, char)> {
    let e = *text.get(p + 1)?;

    if e == b'\\' || (SINGLE && e == b'\'') || (DOUBLE && e == b'"') {
        return Some((2, e as char));
    }

    let hex_start = if CSS { p + 1 } else { p + 2 };

    if (CSS && text.get(p + 3) == Some(&b' ')) || (!CSS && e == b'x') {
        if let (Some(high), Some(low)) = (text.get(hex_start), text.get(hex_start + 1)) {
            if let (Some(high), Some(low)) = (hex_digit(*high), hex_digit(*low)) {
                let value = high * 16 + low;

                if value == b'<' || (value < 32 && (!CSS || value != 0)) {
                    return Some((4, value as char));
                }
            }
        }
    }

    if !CSS {
        if text[p..].starts_with(b"\\u2028") {
            return Some((6, '\u{2028}'));
        }

        if text[p..].starts_with(b"\\u2029") {
            return Some((6, '\u{2029}'));
        }
    }

    // Check the original input so decoded text is never decoded again.
    if p > 0 && text[p - 1] == b'<' {
        if text[(p + 1)..].starts_with(b"!--") {
            return Some((2, '!'));
        }

        if e == b'/' {
            let name: &[u8] = if CSS { b"style" } else { b"script" };
            let name_start = p + 2;
            let name_end = name_start + name.len();

            if text.get(name_start..name_end)?.eq_ignore_ascii_case(name)
                && matches!(text.get(name_end), Some(b'/' | b'>' | 9..=13 | 28..=32))
            {
                return Some((2, '/'));
            }
        }
    }

    None
}

#[inline]
pub(super) fn decode<const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &str,
) -> Cow<'_, str> {
    let text_bytes = text.as_bytes();
    let mut p = 0;

    let (consumed, character) = loop {
        if p == text_bytes.len() {
            return Cow::Borrowed(text);
        }

        if text_bytes[p] == b'\\' {
            if let Some(escape) = decode_escape::<CSS, SINGLE, DOUBLE>(text_bytes, p) {
                break escape;
            }
        }

        p += 1;
    };

    let mut output = Vec::with_capacity(text_bytes.len());

    output.extend_from_slice(&text_bytes[..p]);
    write_char_to_vec(character, &mut output);

    // SAFETY: every decoded escape ends after ASCII bytes.
    decode_to_vec::<CSS, SINGLE, DOUBLE>(
        unsafe { from_utf8_unchecked(&text_bytes[(p + consumed)..]) },
        &mut output,
    );

    // SAFETY: input slices end at ASCII bytes, and each decoded value is a valid character.
    Cow::Owned(unsafe { String::from_utf8_unchecked(output) })
}

#[inline]
pub(super) fn decode_to_vec<'a, const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &str,
    output: &'a mut Vec<u8>,
) -> &'a [u8] {
    let text_bytes = text.as_bytes();
    output.reserve(text_bytes.len());

    let current_length = output.len();
    let mut start = 0;
    let mut p = 0;

    while p < text_bytes.len() {
        if text_bytes[p] != b'\\' {
            p += 1;

            continue;
        }

        if let Some((consumed, character)) = decode_escape::<CSS, SINGLE, DOUBLE>(text_bytes, p) {
            output.extend_from_slice(&text_bytes[start..p]);
            write_char_to_vec(character, output);
            p += consumed;
            start = p;
        } else {
            p += 1;
        }
    }

    output.extend_from_slice(&text_bytes[start..]);

    &output[current_length..]
}

#[cfg(feature = "std")]
#[inline]
pub(super) fn decode_to_writer<
    W: Write + ?Sized,
    const CSS: bool,
    const SINGLE: bool,
    const DOUBLE: bool,
>(
    text: &str,
    output: &mut W,
) -> Result<(), io::Error> {
    let text_bytes = text.as_bytes();
    let mut start = 0;
    let mut p = 0;

    while p < text_bytes.len() {
        if text_bytes[p] != b'\\' {
            p += 1;

            continue;
        }

        if let Some((consumed, character)) = decode_escape::<CSS, SINGLE, DOUBLE>(text_bytes, p) {
            output.write_all(&text_bytes[start..p])?;
            write_char_to_writer(character, output)?;
            p += consumed;
            start = p;
        } else {
            p += 1;
        }
    }

    output.write_all(&text_bytes[start..])
}
