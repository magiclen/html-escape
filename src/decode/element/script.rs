use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

macro_rules! parse_script {
    (@eof $step:ident, $b:block) => {
        // The text ends right after `<\/script`.
        if $step == 9 {
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
                b'c' | b'C' => $step = 5,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            5 => match $e {
                b'r' | b'R' => $step = 6,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            6 => match $e {
                b'i' | b'I' => $step = 7,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            7 => match $e {
                b'p' | b'P' => $step = 8,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            8 => match $e {
                b't' | b'T' => $step = 9,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            9 => match $e {
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
    7;
    /// The following substring is unescaped:
    ///
    /// * `<\/script>` => `</script>`
    /// * `<\!--` => `<!--`
    ///
    /// `<\/script` at the end of the text is also unescaped.
    parse_script;
    /// Decode text from the `<script>` element.
    decode_script;
    /// Write text from the `<script>` element to a mutable `String` reference and return the decoded string slice.
    decode_script_to_string;
    /// Write text from the `<script>` element to a mutable `Vec<u8>` reference and return the decoded data slice.
    decode_script_to_vec;
    /// Write text from the `<script>` element to a writer.
    decode_script_to_writer;
}

decode_impl! {
    quoted false, true, false;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes use `\xHH` with uppercase digits; only escaped C0 controls and `<` are decoded.
    /// `\u2028` and `\u2029` are also decoded.
    /// Backslashes and `'` escaped with a backslash are decoded.
    /// Legacy `<\/script>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a single-quoted JavaScript string from `<script>`.
    decode_script_single_quoted_text;
    /// Decode content of a single-quoted JavaScript string to a mutable `String` and return the new string slice.
    decode_script_single_quoted_text_to_string;
    /// Decode content of a single-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    decode_script_single_quoted_text_to_vec;
    /// Decode content of a single-quoted JavaScript string to a writer.
    decode_script_single_quoted_text_to_writer;
}

decode_impl! {
    quoted false, false, true;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes use `\xHH` with uppercase digits; only escaped C0 controls and `<` are decoded.
    /// `\u2028` and `\u2029` are also decoded.
    /// Backslashes and `"` escaped with a backslash are decoded.
    /// Legacy `<\/script>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a double-quoted JavaScript string from `<script>`.
    decode_script_double_quoted_text;
    /// Decode content of a double-quoted JavaScript string to a mutable `String` and return the new string slice.
    decode_script_double_quoted_text_to_string;
    /// Decode content of a double-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    decode_script_double_quoted_text_to_vec;
    /// Decode content of a double-quoted JavaScript string to a writer.
    decode_script_double_quoted_text_to_writer;
}

decode_impl! {
    quoted false, true, true;
    /// Escapes emitted by the matching encoder are decoded.
    /// Hex escapes use `\xHH` with uppercase digits; only escaped C0 controls and `<` are decoded.
    /// `\u2028` and `\u2029` are also decoded.
    /// Backslashes, `'`, and `"` escaped with a backslash are decoded.
    /// Legacy `<\/script>` and `<\!--` escapes are also decoded.
    /// Unknown or incomplete escapes are kept, and decoded text is not decoded again.
    raw_text;
    /// Decode content of a single-quoted or double-quoted JavaScript string from `<script>`.
    decode_script_quoted_text;
    /// Decode content of a single-quoted or double-quoted JavaScript string to a mutable `String` and return the new string slice.
    decode_script_quoted_text_to_string;
    /// Decode content of a single-quoted or double-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    decode_script_quoted_text_to_vec;
    /// Decode content of a single-quoted or double-quoted JavaScript string to a writer.
    decode_script_quoted_text_to_writer;
}
