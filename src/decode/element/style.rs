use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

macro_rules! parse_style {
    (@eof $step:ident, $b:block) => {
        // The text ends right after `<\/style`.
        if $step == 8 {
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
                b'\\' => $step = 2,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            2 => match $e {
                b'/' => $step = 3,
                b'!' => $step = 10,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            3 => match $e {
                b's' | b'S' => $step = 4,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            4 => match $e {
                b't' | b'T' => $step = 5,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            5 => match $e {
                b'y' | b'Y' => $step = 6,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            6 => match $e {
                b'l' | b'L' => $step = 7,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            7 => match $e {
                b'e' | b'E' => $step = 8,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            8 => match $e {
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

decode_impl! {
    6;
    /// The following substring is unescaped:
    ///
    /// * `<\/style>` => `</style>`
    /// * `<\!--` => `<!--`
    ///
    /// `<\/style` at the end of the text is also unescaped.
    parse_style;
    /// Decode text from the `<style>` element.
    decode_style;
    /// Write text from the `<style>` element to a mutable `String` reference and return the decoded string slice.
    decode_style_to_string;
    /// Write text from the `<style>` element to a mutable `Vec<u8>` reference and return the decoded data slice.
    decode_style_to_vec;
    /// Write text from the `<style>` element to a writer.
    decode_style_to_writer;
}

decode_impl! {
    quoted true, true, false;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes need two uppercase digits and one ending space; only escaped C0 controls and `<` are decoded.
    /// Only the ending space is removed; an input space after it is kept.
    /// U+FFFD from an encoded NUL is kept as U+FFFD.
    /// Backslashes and `'` escaped with a backslash are decoded.
    /// Legacy `<\/style>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a single-quoted CSS string from `<style>`.
    decode_style_single_quoted_text;
    /// Decode content of a single-quoted CSS string to a mutable `String` and return the new string slice.
    decode_style_single_quoted_text_to_string;
    /// Decode content of a single-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    decode_style_single_quoted_text_to_vec;
    /// Decode content of a single-quoted CSS string to a writer.
    decode_style_single_quoted_text_to_writer;
}

decode_impl! {
    quoted true, false, true;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes need two uppercase digits and one ending space; only escaped C0 controls and `<` are decoded.
    /// Only the ending space is removed; an input space after it is kept.
    /// U+FFFD from an encoded NUL is kept as U+FFFD.
    /// Backslashes and `"` escaped with a backslash are decoded.
    /// Legacy `<\/style>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a double-quoted CSS string from `<style>`.
    decode_style_double_quoted_text;
    /// Decode content of a double-quoted CSS string to a mutable `String` and return the new string slice.
    decode_style_double_quoted_text_to_string;
    /// Decode content of a double-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    decode_style_double_quoted_text_to_vec;
    /// Decode content of a double-quoted CSS string to a writer.
    decode_style_double_quoted_text_to_writer;
}

decode_impl! {
    quoted true, true, true;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes need two uppercase digits and one ending space; only escaped C0 controls and `<` are decoded.
    /// Only the ending space is removed; an input space after it is kept.
    /// U+FFFD from an encoded NUL is kept as U+FFFD.
    /// Backslashes, `'`, and `"` escaped with a backslash are decoded.
    /// Legacy `<\/style>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a single-quoted or double-quoted CSS string from `<style>`.
    decode_style_quoted_text;
    /// Decode content of a single-quoted or double-quoted CSS string to a mutable `String` and return the new string slice.
    decode_style_quoted_text_to_string;
    /// Decode content of a single-quoted or double-quoted CSS string to a mutable `Vec<u8>` and return the new data slice.
    decode_style_quoted_text_to_vec;
    /// Decode content of a single-quoted or double-quoted CSS string to a writer.
    decode_style_quoted_text_to_writer;
}
