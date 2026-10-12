use alloc::{borrow::Cow, string::String, vec::Vec};
use core::str::from_utf8_unchecked;
#[cfg(feature = "std")]
use std::io::{self, Write};

macro_rules! parse_script_comment {
    (@eof $step:ident, $b:block) => {
        // An unfinished `</script` at the end can still end the element with the text after it.
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
                b'c' | b'C' => $step = 4,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            4 => match $e {
                b'r' | b'R' => $step = 5,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            5 => match $e {
                b'i' | b'I' => $step = 6,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            6 => match $e {
                b'p' | b'P' => $step = 7,
                b'<' => $step = 1,
                _ => $step = 0,
            },
            7 => match $e {
                b't' | b'T' => $step = 8,
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

encode_impl! {
    7;
    /// The following substring is escaped:
    ///
    /// * `</script>` => `<\/script>`
    /// * `<!--` => `<\!--`
    ///
    /// `</script` at the end of the text is also escaped, because the text after it can end the element.
    /// Pass the whole element text in one call; separate calls do not share parsing state.
    parse_script_comment;
    /// Encode text used in the `<script>` element.
    encode_script;
    /// Write text used in the `<script>` element to a mutable `String` reference and return the encoded string slice.
    encode_script_to_string;
    /// Write text used in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice.
    encode_script_to_vec;
    /// Write text used in the `<script>` element to a writer.
    encode_script_to_writer;
}

encode_impl! {
    quoted false, true, false;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes and `'` are escaped with a backslash.
    /// `<` and C0 controls use `\xHH` with uppercase hex digits.
    /// U+2028 and U+2029 use `\u2028` and `\u2029`.
    /// Use the output in a JavaScript string inside `<script>`, not in a template literal or an HTML attribute.
    raw_text;
    /// Encode raw text for a single-quoted JavaScript string inside `<script>`.
    encode_script_single_quoted_text;
    /// Encode content of a single-quoted JavaScript string to a mutable `String` and return the new string slice.
    encode_script_single_quoted_text_to_string;
    /// Encode content of a single-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    encode_script_single_quoted_text_to_vec;
    /// Encode content of a single-quoted JavaScript string to a writer.
    encode_script_single_quoted_text_to_writer;
}

encode_impl! {
    quoted false, false, true;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes and `"` are escaped with a backslash.
    /// `<` and C0 controls use `\xHH` with uppercase hex digits.
    /// U+2028 and U+2029 use `\u2028` and `\u2029`.
    /// Use the output in a JavaScript string inside `<script>`, not in a template literal or an HTML attribute.
    raw_text;
    /// Encode raw text for a double-quoted JavaScript string inside `<script>`.
    encode_script_double_quoted_text;
    /// Encode content of a double-quoted JavaScript string to a mutable `String` and return the new string slice.
    encode_script_double_quoted_text_to_string;
    /// Encode content of a double-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    encode_script_double_quoted_text_to_vec;
    /// Encode content of a double-quoted JavaScript string to a writer.
    encode_script_double_quoted_text_to_writer;
}

encode_impl! {
    quoted false, true, true;
    /// The input is raw text; existing escapes are encoded as text.
    /// The output is string content and does not include outer quotes.
    /// Backslashes, `'`, and `"` are escaped with a backslash.
    /// `<` and C0 controls use `\xHH` with uppercase hex digits.
    /// U+2028 and U+2029 use `\u2028` and `\u2029`.
    /// Use the output in a JavaScript string inside `<script>`, not in a template literal or an HTML attribute.
    raw_text;
    /// Encode raw text for a single-quoted or double-quoted JavaScript string inside `<script>`.
    encode_script_quoted_text;
    /// Encode content of a single-quoted or double-quoted JavaScript string to a mutable `String` and return the new string slice.
    encode_script_quoted_text_to_string;
    /// Encode content of a single-quoted or double-quoted JavaScript string to a mutable `Vec<u8>` and return the new data slice.
    encode_script_quoted_text_to_vec;
    /// Encode content of a single-quoted or double-quoted JavaScript string to a writer.
    encode_script_quoted_text_to_writer;
}
