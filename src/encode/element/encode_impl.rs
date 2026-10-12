macro_rules! encode_impl {
    (quoted $css:literal, $single:literal, $double:literal; $(#[$attr:meta])* raw_text; $(#[$encode_attr:meta])* $encode_name:ident; $(#[$encode_to_string_attr:meta])* $encode_to_string_name:ident; $(#[$encode_to_vec_attr:meta])* $encode_to_vec_name:ident; $(#[$encode_to_writer_attr:meta])* $encode_to_writer_name:ident $(;)*) => {
        $(#[$encode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
            super::quoted::encode::<$css, $single, $double>(text.as_ref())
        }

        $(#[$encode_to_string_attr])*
        ///
        /// This appends to `output` and returns only the new string slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            // SAFETY: the encoded data is valid UTF-8, so `output` stays a valid `String`.
            unsafe { from_utf8_unchecked($encode_to_vec_name(text, output.as_mut_vec())) }
        }

        $(#[$encode_to_vec_attr])*
        ///
        /// This appends to `output` and returns only the new data slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_vec_name<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
            super::quoted::encode_to_vec::<$css, $single, $double>(text.as_ref(), output)
        }

        #[cfg(feature = "std")]
        $(#[$encode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_writer_name<S: AsRef<str>, W: Write + ?Sized>(text: S, output: &mut W) -> Result<(), io::Error> {
            super::quoted::encode_to_writer::<W, $css, $single, $double>(text.as_ref(), output)
        }
    };
    ($l:expr; $(#[$attr: meta])* $parse_macro:ident; $(#[$encode_attr: meta])* $encode_name: ident; $(#[$encode_to_string_attr: meta])* $encode_to_string_name: ident; $(#[$encode_to_vec_attr: meta])* $encode_to_vec_name: ident; $(#[$encode_to_writer_attr: meta])* $encode_to_writer_name: ident $(;)*) => {
        $(#[$encode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();
            let text_length = text_bytes.len();

            let mut p = 0;

            let mut step = 0;

            let (mut v, mut start) = 'scan: loop {
                // Only `<` can leave state 0, so the other bytes are skipped.
                p = crate::functions::find_less_than(text_bytes, p);

                if p == text_length {
                    $parse_macro!(@eof step, {
                        let mut v = Vec::with_capacity(text_length + 1);

                        v.extend_from_slice(&text_bytes[..(p - $l)]);

                        break (v, p - $l);
                    });

                    return Cow::from(text);
                }

                for e in text_bytes[p..].iter().copied() {
                    $parse_macro!(
                        e,
                        step,
                        {
                            let mut v = Vec::with_capacity(text_length + 1);

                            v.extend_from_slice(&text_bytes[..(p - $l)]);

                            break 'scan (v, p - $l);
                        },
                        {
                            let mut v = Vec::with_capacity(text_length + 1);

                            v.extend_from_slice(&text_bytes[..(p - 2)]);

                            break 'scan (v, p - 2);
                        }
                    );

                    p += 1;

                    if step == 0 {
                        break;
                    }
                }
            };

            v.push(b'\\');

            // `p` is already at the end after an unfinished end tag at the end.
            if p < text_length {
                p += 1;

                loop {
                    p = crate::functions::find_less_than(text_bytes, p);

                    if p == text_length {
                        break;
                    }

                    for e in text_bytes[p..].iter().copied() {
                        $parse_macro!(
                            e,
                            step,
                            {
                                v.extend_from_slice(&text_bytes[start..(p - $l)]);
                                start = p - $l;
                                v.push(b'\\');
                            },
                            {
                                v.extend_from_slice(&text_bytes[start..(p - 2)]);
                                start = p - 2;
                                v.push(b'\\');
                            }
                        );

                        p += 1;

                        if step == 0 {
                            break;
                        }
                    }
                }

                $parse_macro!(@eof step, {
                    v.extend_from_slice(&text_bytes[start..(p - $l)]);
                    start = p - $l;
                    v.push(b'\\');
                });
            }

            v.extend_from_slice(&text_bytes[start..p]);

            // SAFETY: `v` only contains slices of `text`, split at ASCII bytes, plus the ASCII byte `\`, so it is valid UTF-8.
            Cow::from(unsafe { String::from_utf8_unchecked(v) })
        }

        $(#[$encode_to_string_attr])*
        ///
        /// This appends to `output` and returns only the new string slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            // SAFETY: the encoded data is valid UTF-8, so `output` remains a valid `String`.
            unsafe { from_utf8_unchecked($encode_to_vec_name(text, output.as_mut_vec())) }
        }

        $(#[$encode_to_vec_attr])*
        ///
        /// This appends to `output` and returns only the new data slice.
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

            let mut step = 0;

            loop {
                // Only `<` can leave state 0, so the other bytes are skipped.
                end = crate::functions::find_less_than(text_bytes, end);

                if end == text_length {
                    break;
                }

                for e in text_bytes[end..].iter().copied() {
                    $parse_macro!(
                        e,
                        step,
                        {
                            output.extend_from_slice(&text_bytes[start..(end - $l)]);
                            start = end - $l;
                            output.push(b'\\');
                        },
                        {
                            output.extend_from_slice(&text_bytes[start..(end - 2)]);
                            start = end - 2;
                            output.push(b'\\');
                        }
                    );

                    end += 1;

                    if step == 0 {
                        break;
                    }
                }
            }

            $parse_macro!(@eof step, {
                output.extend_from_slice(&text_bytes[start..(end - $l)]);
                start = end - $l;
                output.push(b'\\');
            });

            output.extend_from_slice(&text_bytes[start..end]);

            &output[current_length..]
        }

        #[cfg(feature = "std")]
        $(#[$encode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $encode_to_writer_name<S: AsRef<str>, W: Write + ?Sized>(text: S, output: &mut W) -> Result<(), io::Error> {
            let text = text.as_ref();
            let text_bytes = text.as_bytes();
            let text_length = text_bytes.len();

            let mut start = 0;
            let mut end = 0;

            let mut step = 0;

            loop {
                // Only `<` can leave state 0, so the other bytes are skipped.
                end = crate::functions::find_less_than(text_bytes, end);

                if end == text_length {
                    break;
                }

                for e in text_bytes[end..].iter().copied() {
                    $parse_macro!(
                        e,
                        step,
                        {
                            output.write_all(&text_bytes[start..(end - $l)])?;
                            start = end - $l;
                            output.write_all(b"\\")?;
                        },
                        {
                            output.write_all(&text_bytes[start..(end - 2)])?;
                            start = end - 2;
                            output.write_all(b"\\")?;
                        }
                    );

                    end += 1;

                    if step == 0 {
                        break;
                    }
                }
            }

            $parse_macro!(@eof step, {
                output.write_all(&text_bytes[start..(end - $l)])?;
                start = end - $l;
                output.write_all(b"\\")?;
            });

            output.write_all(&text_bytes[start..end])
        }
    };
}
