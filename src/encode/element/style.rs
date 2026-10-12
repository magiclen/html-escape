use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

macro_rules! parse_style_comment {
    (@eof $step:ident, $b:block) => {
        // An unfinished `</style` at the end can still end the element with the text after it.
        if $step == 7 {
            $b
        }
    };
    ($e:expr, $step:ident, $b:block, $bc:block) => {
        match $step {
            0 => match $e {
                b'<' => $step = 1,
                _ => (),
            },
            1 => match $e {
                b'/' => $step = 2,
                b'!' => $step = 10,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            2 => match $e {
                b's' | b'S' => $step = 3,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            3 => match $e {
                b't' | b'T' => $step = 4,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            4 => match $e {
                b'y' | b'Y' => $step = 5,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            5 => match $e {
                b'l' | b'L' => $step = 6,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            6 => match $e {
                b'e' | b'E' => $step = 7,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            7 => match $e {
                b'/' | b'>' | 9..=13 | 28..=32 => {
                    $step = 0;
                    $b
                },
                b'<' => $step = 1,
                _ => $step = 0,
            },
            10 => match $e {
                b'-' => $step = 11,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            11 => match $e {
                b'-' => {
                    $step = 0;
                    $bc
                },
                b'<' => $step = 1,
                _ => $step = 0,
            },
            _ => unreachable!(),
        }
    };
}

encode_impl! {
    6;
    /// The following substring is escaped:
    ///
    /// * `</style>` => `<\/style>`
    /// * `<!--` => `<\!--`
    ///
    /// `</style` at the end of the text is also escaped, because the text after it can end the element.
    /// Pass the whole element text in one call; separate calls do not share parsing state.
    parse_style_comment;
    /// Encode text used in the `<style>` element.
    encode_style;
    /// Write text used in the `<style>` element to a mutable `String` reference and return the encoded string slice.
    encode_style_to_string;
    /// Write text used in the `<style>` element to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_style_to_vec;
    /// Write text used in the `<style>` element to a writer.
    encode_style_to_writer;
}

encode_impl! {
    quoted true, true, false;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes and `'` are escaped with a backslash.
    /// `<` and C0 controls other than NUL use two uppercase hex digits and one ending space, such as `\3C `.
    /// NUL is replaced with U+FFFD, so text containing NUL cannot round-trip without a change.
    /// The ending space is part of the escape; an input space after it is kept.
    /// Use the output in a CSS string inside `<style>`, not in a CSS identifier or an HTML attribute.
    raw_text;
    /// Encode raw text for a single-quoted CSS string inside `<style>`.
    encode_style_single_quoted_text;
    /// Encode content of a single-quoted CSS string to a mutable `String` and return the new string slice.
    encode_style_single_quoted_text_to_string;
    /// Encode content of a single-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    encode_style_single_quoted_text_to_vec;
    /// Encode content of a single-quoted CSS string to a writer.
    encode_style_single_quoted_text_to_writer;
}

encode_impl! {
    quoted true, false, true;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes and `"` are escaped with a backslash.
    /// `<` and C0 controls other than NUL use two uppercase hex digits and one ending space, such as `\3C `.
    /// NUL is replaced with U+FFFD, so text containing NUL cannot round-trip without a change.
    /// The ending space is part of the escape; an input space after it is kept.
    /// Use the output in a CSS string inside `<style>`, not in a CSS identifier or an HTML attribute.
    raw_text;
    /// Encode raw text for a double-quoted CSS string inside `<style>`.
    encode_style_double_quoted_text;
    /// Encode content of a double-quoted CSS string to a mutable `String` and return the new string slice.
    encode_style_double_quoted_text_to_string;
    /// Encode content of a double-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    encode_style_double_quoted_text_to_vec;
    /// Encode content of a double-quoted CSS string to a writer.
    encode_style_double_quoted_text_to_writer;
}

encode_impl! {
    quoted true, true, true;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes, `'`, and `"` are escaped with a backslash.
    /// `<` and C0 controls other than NUL use two uppercase hex digits and one ending space, such as `\3C `.
    /// NUL is replaced with U+FFFD, so text containing NUL cannot round-trip without a change.
    /// The ending space is part of the escape; an input space after it is kept.
    /// Use the output in a CSS string inside `<style>`, not in a CSS identifier or an HTML attribute.
    raw_text;
    /// Encode raw text for a single-quoted or double-quoted CSS string inside `<style>`.
    encode_style_quoted_text;
    /// Encode content of a single-quoted or double-quoted CSS string to a mutable `String` and return the new string slice.
    encode_style_quoted_text_to_string;
    /// Encode content of a single-quoted or double-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    encode_style_quoted_text_to_vec;
    /// Encode content of a single-quoted or double-quoted CSS string to a writer.
    encode_style_quoted_text_to_writer;
}
