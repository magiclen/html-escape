macro_rules! decode_impl {
    (quoted $css:literal, $single:literal, $double:literal; $(#[$attr:meta])* raw_text; $(#[$decode_attr:meta])* $decode_name:ident; $(#[$decode_to_string_attr:meta])* $decode_to_string_name:ident; $(#[$decode_to_vec_attr:meta])* $decode_to_vec_name:ident; $(#[$decode_to_writer_attr:meta])* $decode_to_writer_name:ident $(;)*) => {
        $(#[$decode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
            super::quoted::decode::<$css, $single, $double>(text.as_ref())
        }

        $(#[$decode_to_string_attr])*
        ///
        /// This appends to `output` and returns only the new string slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            // SAFETY: the decoded data is valid UTF-8, so `output` stays a valid `String`.
            unsafe { from_utf8_unchecked($decode_to_vec_name(text, output.as_mut_vec())) }
        }

        $(#[$decode_to_vec_attr])*
        ///
        /// This appends to `output` and returns only the new data slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_vec_name<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
            super::quoted::decode_to_vec::<$css, $single, $double>(text.as_ref(), output)
        }

        #[cfg(feature = "std")]
        $(#[$decode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_writer_name<S: AsRef<str>, W: Write + ?Sized>(text: S, output: &mut W) -> Result<(), io::Error> {
            super::quoted::decode_to_writer::<W, $css, $single, $double>(text.as_ref(), output)
        }
    };
    ($l:expr; $(#[$attr: meta])* $parse_macro:ident; $(#[$decode_attr: meta])* $decode_name: ident; $(#[$decode_to_string_attr: meta])* $decode_to_string_name: ident; $(#[$decode_to_vec_attr: meta])* $decode_to_vec_name: ident; $(#[$decode_to_writer_attr: meta])* $decode_to_writer_name: ident $(;)*) => {
        $(#[$decode_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_name<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
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
                        let mut v = Vec::with_capacity(text_length);

                        v.extend_from_slice(&text_bytes[..(p - ($l + 1))]);

                        break (v, p - $l);
                    });

                    return Cow::from(text);
                }

                for e in text_bytes[p..].iter().copied() {
                    $parse_macro!(e, step,
                        {
                            let mut v = Vec::with_capacity(text_length);

                            v.extend_from_slice(&text_bytes[..(p - ($l + 1))]);

                            break 'scan (v, p - $l);
                        },
                        {
                            let mut v = Vec::with_capacity(text_length);

                            v.extend_from_slice(&text_bytes[..(p - 3)]);

                            break 'scan (v, p - 2);
                        }
                    );

                    p += 1;

                    if step == 0 {
                        break;
                    }
                }
            };

            // `p` is already at the end after an escaped end tag at the end.
            if p < text_length {
                p += 1;

                loop {
                    p = crate::functions::find_less_than(text_bytes, p);

                    if p == text_length {
                        break;
                    }

                    for e in text_bytes[p..].iter().copied() {
                        $parse_macro!(e, step,
                            {
                                v.extend_from_slice(&text_bytes[start..(p - ($l + 1))]);
                                start = p - $l;
                            },
                            {
                                v.extend_from_slice(&text_bytes[start..(p - 3)]);
                                start = p - 2;
                            }
                        );

                        p += 1;

                        if step == 0 {
                            break;
                        }
                    }
                }

                $parse_macro!(@eof step, {
                    v.extend_from_slice(&text_bytes[start..(p - ($l + 1))]);
                    start = p - $l;
                });
            }

            v.extend_from_slice(&text_bytes[start..p]);

            // SAFETY: `v` only contains slices of `text`, split at ASCII bytes, so it is valid UTF-8.
            Cow::from(unsafe { String::from_utf8_unchecked(v) })
        }

        $(#[$decode_to_string_attr])*
        ///
        /// This appends to `output` and returns only the new string slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_string_name<S: AsRef<str>>(text: S, output: &mut String) -> &str {
            // SAFETY: the decoded data is valid UTF-8, so `output` remains a valid `String`.
            unsafe { from_utf8_unchecked($decode_to_vec_name(text, output.as_mut_vec())) }
        }

        $(#[$decode_to_vec_attr])*
        ///
        /// This appends to `output` and returns only the new data slice.
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_vec_name<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
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
                    $parse_macro!(e, step,
                        {
                            output.extend_from_slice(&text_bytes[start..(end - ($l + 1))]);
                            start = end - $l;
                        },
                        {
                            output.extend_from_slice(&text_bytes[start..(end - 3)]);
                            start = end - 2;
                        }
                    );

                    end += 1;

                    if step == 0 {
                        break;
                    }
                }
            }

            $parse_macro!(@eof step, {
                output.extend_from_slice(&text_bytes[start..(end - ($l + 1))]);
                start = end - $l;
            });

            output.extend_from_slice(&text_bytes[start..end]);

            &output[current_length..]
        }

        #[cfg(feature = "std")]
        $(#[$decode_to_writer_attr])*
        ///
        $(#[$attr])*
        #[inline]
        pub fn $decode_to_writer_name<S: AsRef<str>, W: Write + ?Sized>(text: S, output: &mut W) -> Result<(), io::Error> {
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
                    $parse_macro!(e, step,
                        {
                            output.write_all(&text_bytes[start..(end - ($l + 1))])?;
                            start = end - $l;
                        },
                        {
                            output.write_all(&text_bytes[start..(end - 3)])?;
                            start = end - 2;
                        }
                    );

                    end += 1;

                    if step == 0 {
                        break;
                    }
                }
            }

            $parse_macro!(@eof step, {
                output.write_all(&text_bytes[start..(end - ($l + 1))])?;
                start = end - $l;
            });

            output.write_all(&text_bytes[start..end])
        }
    };
}
