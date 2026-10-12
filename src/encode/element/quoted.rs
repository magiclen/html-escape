use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::functions::HEX_DIGITS;

/// `\`, C0 controls, and `<` are always escaped.
const ESCAPE_ALWAYS: u8 = 1;
const ESCAPE_SINGLE_QUOTE: u8 = 1 << 1;
const ESCAPE_DOUBLE_QUOTE: u8 = 1 << 2;
/// U+2028 and U+2029 start with the byte `0xE2`, and they are escaped in JavaScript strings.
const ESCAPE_LINE_SEPARATOR: u8 = 1 << 3;

const fn build_escape_flags() -> [u8; 256] {
    let mut flags = [0; 256];
    let mut e = 0;

    while e < 32 {
        flags[e] = ESCAPE_ALWAYS;
        e += 1;
    }

    flags[b'\\' as usize] = ESCAPE_ALWAYS;
    flags[b'<' as usize] = ESCAPE_ALWAYS;
    flags[b'\'' as usize] = ESCAPE_SINGLE_QUOTE;
    flags[b'"' as usize] = ESCAPE_DOUBLE_QUOTE;
    flags[0xE2] = ESCAPE_LINE_SEPARATOR;

    flags
}

static ESCAPE_FLAGS: [u8; 256] = build_escape_flags();

/// Check whether `e` can start an escape, so most bytes do not need to call `escape`.
///
/// A table lookup is used because a `match` here can compile to many unpredictable branches.
#[inline]
fn needs_escape<const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(e: u8) -> bool {
    let mut mask = ESCAPE_ALWAYS;

    if SINGLE {
        mask |= ESCAPE_SINGLE_QUOTE;
    }

    if DOUBLE {
        mask |= ESCAPE_DOUBLE_QUOTE;
    }

    if !CSS {
        mask |= ESCAPE_LINE_SEPARATOR;
    }

    ESCAPE_FLAGS[e as usize] & mask != 0
}

#[inline]
fn escape<const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &[u8],
    p: usize,
    buffer: &mut [u8; 6],
) -> Option<(usize, usize)> {
    let e = text[p];

    match e {
        b'\\' => {
            buffer[..2].copy_from_slice(b"\\\\");
            Some((1, 2))
        },
        b'\'' if SINGLE => {
            buffer[..2].copy_from_slice(b"\\'");
            Some((1, 2))
        },
        b'"' if DOUBLE => {
            buffer[..2].copy_from_slice(b"\\\"");
            Some((1, 2))
        },
        0 if CSS => {
            buffer[..3].copy_from_slice("\u{FFFD}".as_bytes());
            Some((1, 3))
        },
        0..=31 | b'<' => {
            buffer[0] = b'\\';

            if CSS {
                buffer[1] = HEX_DIGITS[(e >> 4) as usize];
                buffer[2] = HEX_DIGITS[(e & 0xF) as usize];
                buffer[3] = b' ';
            } else {
                buffer[1] = b'x';
                buffer[2] = HEX_DIGITS[(e >> 4) as usize];
                buffer[3] = HEX_DIGITS[(e & 0xF) as usize];
            }

            Some((1, 4))
        },
        0xE2 if !CSS && text.get(p + 1) == Some(&0x80) => match text.get(p + 2) {
            Some(0xA8) => {
                buffer.copy_from_slice(b"\\u2028");
                Some((3, 6))
            },
            Some(0xA9) => {
                buffer.copy_from_slice(b"\\u2029");
                Some((3, 6))
            },
            _ => None,
        },
        _ => None,
    }
}

#[inline]
pub(super) fn encode<const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &str,
) -> Cow<'_, str> {
    let text_bytes = text.as_bytes();
    let mut p = 0;
    let mut buffer = [0; 6];

    let (consumed, length) = loop {
        if p == text_bytes.len() {
            return Cow::Borrowed(text);
        }

        if needs_escape::<CSS, SINGLE, DOUBLE>(text_bytes[p]) {
            if let Some(escape) = escape::<CSS, SINGLE, DOUBLE>(text_bytes, p, &mut buffer) {
                break escape;
            }
        }

        p += 1;
    };

    let mut output = Vec::with_capacity(text_bytes.len() + 3);

    output.extend_from_slice(&text_bytes[..p]);
    output.extend_from_slice(&buffer[..length]);

    // SAFETY: the escape consumes one ASCII byte or one whole Unicode character.
    encode_to_vec::<CSS, SINGLE, DOUBLE>(
        unsafe { from_utf8_unchecked(&text_bytes[(p + consumed)..]) },
        &mut output,
    );

    // SAFETY: input slices end at whole characters, and all escapes are valid UTF-8.
    Cow::Owned(unsafe { String::from_utf8_unchecked(output) })
}

#[inline]
pub(super) fn encode_to_vec<'a, const CSS: bool, const SINGLE: bool, const DOUBLE: bool>(
    text: &str,
    output: &'a mut Vec<u8>,
) -> &'a [u8] {
    let text_bytes = text.as_bytes();
    output.reserve(text_bytes.len());

    let current_length = output.len();
    let mut start = 0;
    let mut p = 0;
    let mut buffer = [0; 6];

    while p < text_bytes.len() {
        if !needs_escape::<CSS, SINGLE, DOUBLE>(text_bytes[p]) {
            p += 1;

            continue;
        }

        if let Some((consumed, length)) = escape::<CSS, SINGLE, DOUBLE>(text_bytes, p, &mut buffer)
        {
            output.extend_from_slice(&text_bytes[start..p]);
            output.extend_from_slice(&buffer[..length]);
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
pub(super) fn encode_to_writer<
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
    let mut buffer = [0; 6];

    while p < text_bytes.len() {
        if !needs_escape::<CSS, SINGLE, DOUBLE>(text_bytes[p]) {
            p += 1;

            continue;
        }

        if let Some((consumed, length)) = escape::<CSS, SINGLE, DOUBLE>(text_bytes, p, &mut buffer)
        {
            output.write_all(&text_bytes[start..p])?;
            output.write_all(&buffer[..length])?;
            p += consumed;
            start = p;
        } else {
            p += 1;
        }
    }

    output.write_all(&text_bytes[start..])
}
