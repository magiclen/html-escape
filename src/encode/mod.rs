#[macro_use]
mod escape_impl;

use core::str::from_utf8_unchecked;

#[cfg(feature = "std")]
use std::io::{self, Write};

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::functions::*;
use crate::utf8_width;

macro_rules! encode_impl {
    ($(#[$attr: meta])* $escape_macro:ident; $(#[$encode_attr: meta])* $encode_name: ident; $(#[$encode_to_string_attr: meta])* $encode_to_string_name: ident; $(#[$encode_to_vec_attr: meta])* $encode_to_vec_name: ident; $(#[$encode_to_writer_attr: meta])* $encode_to_writer_name: ident $(;)*) => {
        $(#[$encode_attr])*
        ///
        $(#[$attr])*
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

            p += 1;

            loop {
                if p == text_length {
                    break;
                }

                e = text_bytes[p];

                $escape_macro!(vec e, v);

                p += 1;
            }

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

            for e in text_bytes.iter().copied() {
                $escape_macro!(vec e, output);
            }

            &output[current_length..]
        }

        $(#[$encode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[cfg(feature = "std")]
        #[inline]
        pub fn $encode_to_writer_name<S: AsRef<str>, W: Write>(text: S, output: &mut W) -> Result<(), io::Error> {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();

            for e in text_bytes.iter().copied() {
                $escape_macro!(writer e, output);
            }

            Ok(())
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

    p += 1;

    loop {
        if p == text_length {
            break;
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 {
            if is_alphanumeric(e) {
                v.push(e);
            } else {
                write_html_entity_to_vec(e, &mut v);
            }
        } else {
            v.extend_from_slice(&text_bytes[p..(p + width)]);
        }

        p += width;
    }

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Write text used in an unquoted attribute to a mutable `String` reference and return the encoded data slice. Except for alphanumeric characters, escape all characters which are less than 128.
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

    loop {
        if p == text_length {
            break;
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 {
            if is_alphanumeric(e) {
                output.extend_from_slice(&[e]);
            } else {
                write_html_entity_to_vec(e, output);
            }
        } else {
            output.extend_from_slice(&text_bytes[p..(p + width)]);
        }

        p += width;
    }

    &output[current_length..]
}

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
#[cfg(feature = "std")]
pub fn encode_unquoted_attribute_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let text_length = text_bytes.len();

    let mut p = 0;
    let mut e;

    loop {
        if p == text_length {
            break;
        }

        e = text_bytes[p];

        let width = unsafe { utf8_width::get_width_assume_valid(e) };

        if width == 1 {
            if is_alphanumeric(e) {
                output.write_all(&[e])?;
            } else {
                write_html_entity_to_writer(e, output)?;
            }
        } else {
            output.write_all(&text_bytes[p..(p + width)])?;
        }

        p += width;
    }

    Ok(())
}
