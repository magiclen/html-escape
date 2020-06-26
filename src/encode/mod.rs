#[macro_use]
mod escape_impl;

use core::str::from_utf8_unchecked;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::functions::*;
use crate::utf8_width;

macro_rules! encode_impl {
    ($(#[$attr: meta])* $escape_macro:ident; $(#[$encode_attr: meta])* $encode_name: ident; $(#[$encode_to_string_attr: meta])* $encode_to_string_name: ident; $(#[$encode_to_vec_attr: meta])* $encode_to_vec_name: ident; $(#[$encode_to_writer_attr: meta])* $encode_to_writer_name: ident $(;)*) => {
        $(#[$encode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();
            let text_length = text_bytes.len();

            let mut p = 0;
            let mut e;

            let first = loop {
                if p == text_length {
                    return Cow::from(text);
                }

                e = text_bytes[p];

                $escape_macro!(e);

                p += 1;
            };

            let mut v = Vec::with_capacity(text_length + 5);

            v.extend_from_slice(&text_bytes[..p]);
            v.extend_from_slice(first);

            $encode_to_vec_name(unsafe { text.get_unchecked((p + 1)..) }, &mut v);

            Cow::from(unsafe { String::from_utf8_unchecked(v) })
        }

        $(#[$encode_to_string_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            unsafe { from_utf8_unchecked($encode_to_vec_name(text, output.as_mut_vec())) }
        }

        $(#[$encode_to_vec_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_vec_name<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();
            let text_length = text_bytes.len();

            output.reserve(text_length);

            let current_length = output.len();

            let mut start = 0;
            let mut end = 0;

            for e in text_bytes.iter().copied() {
                $escape_macro!(vec e, output, text_bytes, start, end);
            }

            output.extend_from_slice(&text_bytes[start..end]);

            &output[current_length..]
        }

        #[cfg(feature = "std")]
        $(#[$encode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_writer_name<S: AsRef<str>, W: Write>(text: S, output: &mut W) -> Result<(), io::Error> {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();

            let mut start = 0;
            let mut end = 0;

            for e in text_bytes.iter().copied() {
                $escape_macro!(writer e, output, text_bytes, start, end);
            }

            output.write_all(&text_bytes[start..end])
        }
    };
}

escape_impl! {
    escape_text_minimal;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
}

escape_impl! {
    escape_text;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
    b'>' => b"&gt;",
}

escape_impl! {
    escape_double_quote;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
    b'>' => b"&gt;",
    b'"' => b"&quot;",
}

escape_impl! {
    escape_single_quote;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
    b'>' => b"&gt;",
    b'\'' => b"&#x27;",
}

escape_impl! {
    escape_quote;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
    b'>' => b"&gt;",
    b'"' => b"&quot;",
    b'\'' => b"&#x27;",
}

escape_impl! {
    escape_safe;
    b'&' => b"&amp;",
    b'<' => b"&lt;",
    b'>' => b"&gt;",
    b'"' => b"&quot;",
    b'\'' => b"&#x27;",
    b'/' => b"&#x2F;",
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    escape_text_minimal;
    /// Encode text used as regular HTML text.
    encode_text_minimal;
    /// Write text used as regular HTML text to a mutable `String` reference and return the encoded string slice.
    encode_text_minimal_to_string;
    /// Write text used as regular HTML text to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_text_minimal_to_vec;
    /// Write text used as regular HTML text to a writer.
    encode_text_minimal_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    /// * `>` => `&gt;`
    escape_text;
    /// Encode text used as regular HTML text.
    encode_text;
    /// Write text used as regular HTML text to a mutable `String` reference and return the encoded string slice.
    encode_text_to_string;
    /// Write text used as regular HTML text to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_text_to_vec;
    /// Write text used as regular HTML text to a writer.
    encode_text_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    /// * `>` => `&gt;`
    /// * `"` => `&quot;`
    escape_double_quote;
    /// Encode text used in a double-quoted attribute.
    encode_double_quoted_attribute;
    /// Write text used in a double-quoted attribute to a mutable `String` reference and return the encoded string slice.
    encode_double_quoted_attribute_to_string;
    /// Write text used in a double-quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_double_quoted_attribute_to_vec;
    /// Write text used in a double-quoted attribute to a writer.
    encode_double_quoted_attribute_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    /// * `>` => `&gt;`
    /// * `'` => `&#x27;`
    escape_single_quote;
    /// Encode text used in a single-quoted attribute.
    encode_single_quoted_attribute;
    /// Write text used in a single-quoted attribute to a mutable `String` reference and return the encoded string slice.
    encode_single_quoted_attribute_to_string;
    /// Write text used in a single-quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_single_quoted_attribute_to_vec;
    /// Write text used in a single-quoted attribute to a writer.
    encode_single_quoted_attribute_to_writer;
}

encode_impl! {
    /// The following characters (HTML reserved characters)  are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    /// * `>` => `&gt;`
    /// * `"` => `&quot;`
    /// * `'` => `&#x27;`
    escape_quote;
    /// Encode text used in a quoted attribute.
    encode_quoted_attribute;
    /// Write text used in a quoted attribute to a mutable `String` reference and return the encoded string slice.
    encode_quoted_attribute_to_string;
    /// Write text used in a quoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_quoted_attribute_to_vec;
    /// Write text used in a quoted attribute to a writer.
    encode_quoted_attribute_to_writer;
}

encode_impl! {
    /// The following characters are escaped:
    ///
    /// * `&` => `&amp;`
    /// * `<` => `&lt;`
    /// * `>` => `&gt;`
    /// * `"` => `&quot;`
    /// * `'` => `&#x27;`
    /// * `/` => `&#x2F;`
    escape_safe;
    /// Encode text to prevent special characters functioning.
    encode_safe;
    /// Encode text to prevent special characters functioning and write it to a mutable `String` reference and return the encoded string slice.
    encode_safe_to_string;
    /// Encode text to prevent special characters functioning and write it to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_safe_to_vec;
    /// Encode text to prevent special characters functioning and write it to a writer.
    encode_safe_to_writer;
}

// TODO ----------

/// Encode text used in an unquoted attribute. Except for alphanumeric characters, escape all characters which are less than 128.
///
/// The following characters are escaped to named entities:
///
/// * `&` => `&amp;`
/// * `<` => `&lt;`
/// * `>` => `&gt;`
/// * `"` => `&quot;`
///
/// Other non-alphanumeric characters are escaped to `&#xHH;`.
pub fn encode_unquoted_attribute<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 && !is_alphanumeric(e) {
            break;
        }

        p += width;
    }

    let mut v = Vec::with_capacity(text_length);

    v.extend_from_slice(&text_bytes[..p]);

    write_html_entity_to_vec(e, &mut v);

    encode_unquoted_attribute_to_vec(unsafe { text.get_unchecked((p + 1)..) }, &mut v);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Write text used in an unquoted attribute to a mutable `String` reference and return the encoded string slice. Except for alphanumeric characters, escape all characters which are less than 128.
///
/// The following characters are escaped to named entities:
///
/// * `&` => `&amp;`
/// * `<` => `&lt;`
/// * `>` => `&gt;`
/// * `"` => `&quot;`
///
/// Other non-alphanumeric characters are escaped to `&#xHH;`.
#[inline]
pub fn encode_unquoted_attribute_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(encode_unquoted_attribute_to_vec(text, output.as_mut_vec())) }
}

/// Write text used in an unquoted attribute to a mutable `Vec<u8>` reference and return the encoded data slice. Except for alphanumeric characters, escape all characters which are less than 128.
///
/// The following characters are escaped to named entities:
///
/// * `&` => `&amp;`
/// * `<` => `&lt;`
/// * `>` => `&gt;`
/// * `"` => `&quot;`
///
/// Other non-alphanumeric characters are escaped to `&#xHH;`.
pub fn encode_unquoted_attribute_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut p = 0;
    let mut e;

    let mut start = 0;

    loop {
        if p == text_length {
            break;
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 && !is_alphanumeric(e) {
            output.extend_from_slice(&text_bytes[start..p]);
            start = p + 1;
            write_html_entity_to_vec(e, output);
        }

        p += width;
    }

    output.extend_from_slice(&text_bytes[start..p]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Write text used in an unquoted attribute to a writer. Except for alphanumeric characters, escape all characters which are less than 128.
///
/// The following characters are escaped to named entities:
///
/// * `&` => `&amp;`
/// * `<` => `&lt;`
/// * `>` => `&gt;`
/// * `"` => `&quot;`
///
/// Other non-alphanumeric characters are escaped to `&#xHH;`.
pub fn encode_unquoted_attribute_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    let mut start = 0;

    loop {
        if p == text_length {
            break;
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 && !is_alphanumeric(e) {
            output.write_all(&text_bytes[start..p])?;
            start = p + 1;
            write_html_entity_to_writer(e, output)?;
        }

        p += width;
    }

    output.write_all(&text_bytes[start..p])
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
            _ => unreachable!(),
        }
    };
    ($e:expr, $step:ident) => {
        parse_script!($e, $step, { break; });
    };
}

/// Encode text used in the `<script>` element. Escape `</script>` to `<\/script>`.
pub fn encode_script<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
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

    v.extend_from_slice(&text_bytes[..(p - 7)]);
    v.push(b'\\');

    let mut start = p - 7;

    p += 1;

    for e in text_bytes[p..].iter().copied() {
        parse_script!(e, step, {
            v.extend_from_slice(&text_bytes[start..(p - 7)]);
            start = p + 1;
            v.push(b'\\');
            v.extend_from_slice(&text_bytes[(p - 7)..=p]);
        });

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Write text used in the `<script>` element to a mutable `String` reference and return the encoded string slice. Escape `</script>` to `<\/script>`.
#[inline]
pub fn encode_script_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(encode_script_to_vec(text, output.as_mut_vec())) }
}

/// Write text used in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice. Escape `</script>` to `<\/script>`.
pub fn encode_script_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
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
            output.extend_from_slice(&text_bytes[start..(end - 7)]);
            start = end + 1;
            output.push(b'\\');
            output.extend_from_slice(&text_bytes[(end - 7)..=end]);
        });

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Write text used in the `<script>` element to a writer. Escape `</script>` to `<\/script>`.
pub fn encode_script_to_writer<S: AsRef<str>, W: Write>(
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
            output.write_all(&text_bytes[start..(end - 7)])?;
            start = end + 1;
            output.write_all(b"\\")?;
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
                    b't' | b'T' => $step = 4,
                    _ => $step = 0,
                }
            }
            4 => {
                match $e {
                    b'y' | b'Y' => $step = 5,
                    _ => $step = 0,
                }
            }
            5 => {
                match $e {
                    b'l' | b'L' => $step = 6,
                    _ => $step = 0,
                }
            }
            6 => {
                match $e {
                    b'e' | b'E' => $step = 7,
                    _ => $step = 0,
                }
            }
            7 => {
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

/// Encode text used in the `<style>` element. Escape `</style>` to `<\/style>`.
pub fn encode_style<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
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

    v.extend_from_slice(&text_bytes[..(p - 6)]);
    v.push(b'\\');

    let mut start = p - 6;

    p += 1;

    for e in text_bytes[p..].iter().copied() {
        parse_style!(e, step, {
            v.extend_from_slice(&text_bytes[start..(p - 6)]);
            start = p + 1;
            v.push(b'\\');
            v.extend_from_slice(&text_bytes[(p - 6)..=p]);
        });

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Write text used in the `<style>` element to a mutable `String` reference and return the encoded string slice. Escape `</style>` to `<\/style>`.
#[inline]
pub fn encode_style_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(encode_style_to_vec(text, output.as_mut_vec())) }
}

/// Write text used in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice. Escape `</style>` to `<\/style>`.
pub fn encode_style_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
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
            output.extend_from_slice(&text_bytes[start..(end - 6)]);
            start = end + 1;
            output.push(b'\\');
            output.extend_from_slice(&text_bytes[(end - 6)..=end]);
        });

        end += 1;
    }

    output.extend_from_slice(&text_bytes[start..end]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Write text used in the `<style>` element to a writer. Escape `</style>` to `<\/style>`.
pub fn encode_style_to_writer<S: AsRef<str>, W: Write>(
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
            output.write_all(&text_bytes[start..(end - 6)])?;
            start = end + 1;
            output.write_all(b"\\")?;
            output.write_all(&text_bytes[(end - 6)..=end])?;
        });

        end += 1;
    }

    output.write_all(&text_bytes[start..end])
}
