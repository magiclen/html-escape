use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

#[inline]
pub(crate) fn is_alphanumeric(e: u8) -> bool {
    b'0' <= e && e <= b'9' || b'a' <= e && e <= b'z' || b'A' <= e && e <= b'Z'
}

#[inline]
pub(crate) fn write_hex_to_vec(e: u8, output: &mut Vec<u8>) {
    output.reserve(6);

    let length = output.len();

    unsafe {
        output.set_len(length + 6);
    }

    let output = &mut output[length..];

    output[0] = b'&';
    output[1] = b'#';
    output[2] = b'x';
    output[5] = b';';

    let he = e >> 4;
    let le = e & 0xF;

    output[3] = if he >= 10 {
        b'A' - 10 + he
    } else {
        b'0' + he
    };

    output[4] = if le >= 10 {
        b'A' - 10 + le
    } else {
        b'0' + le
    };
}

#[cfg(feature = "std")]
#[inline]
pub(crate) fn write_hex_to_writer<W: Write>(e: u8, output: &mut W) -> Result<(), io::Error> {
    output.write_fmt(format_args!("&#x{:02X};", e))
}

#[inline]
pub(crate) fn write_html_entity_to_vec(e: u8, output: &mut Vec<u8>) {
    match e {
        b'&' => output.extend_from_slice(b"&amp;"),
        b'<' => output.extend_from_slice(b"&lt;"),
        b'>' => output.extend_from_slice(b"&gt;"),
        b'"' => output.extend_from_slice(b"&quot;"),
        _ => write_hex_to_vec(e, output),
    }
}

#[cfg(feature = "std")]
#[inline]
pub(crate) fn write_html_entity_to_writer<W: Write>(
    e: u8,
    output: &mut W,
) -> Result<(), io::Error> {
    match e {
        b'&' => output.write_all(b"&amp;"),
        b'<' => output.write_all(b"&lt;"),
        b'>' => output.write_all(b"&gt;"),
        b'"' => output.write_all(b"&quot;"),
        _ => write_hex_to_writer(e, output),
    }
}

#[inline]
pub(crate) fn write_char_to_vec(c: char, output: &mut Vec<u8>) {
    let width = c.len_utf8();

    output.reserve(width);

    let current_length = output.len();

    unsafe {
        output.set_len(current_length + width);
    }

    c.encode_utf8(&mut output[current_length..]);
}

#[cfg(feature = "std")]
#[inline]
pub(crate) fn write_char_to_writer<W: Write>(c: char, output: &mut W) -> Result<(), io::Error> {
    let mut buffer = [0u8; 4];
    let length = c.encode_utf8(&mut buffer).len();

    output.write_all(&buffer[..length])
}
