mod named_entities;

use core::convert::TryFrom;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::functions::*;

pub use named_entities::*;

pub fn decode<S: ?Sized + AsRef<str>>(text: &S) -> Cow<str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = 0;
    let mut ep = 0;
    let mut e;

    let mut step = 0;

    let (mut v, mut start) = loop {
        if p == text_length {
            return Cow::from(text);
        }

        e = text_bytes[p];

        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = p;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let name = &text[(ep + 1)..p];

                    match NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name)) {
                        Ok(index) => {
                            v.extend_from_slice(NAMED_ENTITIES[index].1.as_bytes());
                            break (v, p + 1);
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let number = &text[(ep + 2)..p];

                    match number.parse::<u32>() {
                        Ok(number) => {
                            match char::try_from(number) {
                                Ok(c) => {
                                    write_char_to_vec(c, &mut v);
                                    break (v, p + 1);
                                }
                                Err(_) => break (v, ep),
                            }
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    let mut v = Vec::with_capacity(text_length);

                    v.extend_from_slice(&text_bytes[..ep]);

                    let hex = &text[(ep + 3)..p];

                    match u32::from_str_radix(hex, 16) {
                        Ok(number) => {
                            match char::try_from(number) {
                                Ok(c) => {
                                    write_char_to_vec(c, &mut v);
                                    break (v, p + 1);
                                }
                                Err(_) => break (v, ep),
                            }
                        }
                        Err(_) => break (v, ep),
                    }
                }
            }
            _ => unreachable!(),
        }

        p += 1;
    };

    p += 1;

    step = 0;

    for e in text_bytes[p..].iter().copied() {
        match step {
            0 => {
                if e == b'&' {
                    step = 1;
                    ep = p;
                }
            }
            1 => {
                match e {
                    b'#' => {
                        step = 3;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => {
                        step = 2;
                    }
                }
            }
            2 => {
                if e == b';' {
                    // named
                    step = 0;

                    let name = &text[(ep + 1)..p];

                    if let Ok(index) =
                        NAMED_ENTITIES.binary_search_by(|(t_name, _)| t_name.cmp(&name))
                    {
                        v.extend_from_slice(&text_bytes[start..ep]);
                        start = p + 1;
                        v.extend_from_slice(NAMED_ENTITIES[index].1.as_bytes());
                    }
                }
            }
            3 => {
                match e {
                    b'x' | b'X' => {
                        step = 5;
                    }
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => step = 4,
                }
            }
            4 => {
                if e == b';' {
                    // numeric
                    step = 0;

                    let number = &text[(ep + 2)..p];

                    if let Ok(number) = number.parse::<u32>() {
                        if let Ok(c) = char::try_from(number) {
                            v.extend_from_slice(&text_bytes[start..ep]);
                            start = p + 1;
                            write_char_to_vec(c, &mut v);
                        }
                    }
                }
            }
            5 => {
                match e {
                    b';' => {
                        // incorrect
                        step = 0;

                        v.extend_from_slice(&text_bytes[start..=p]);
                        start = p + 1;
                    }
                    _ => step = 6,
                }
            }
            6 => {
                if e == b';' {
                    // hex
                    let hex = &text[(ep + 3)..p];

                    if let Ok(number) = u32::from_str_radix(hex, 16) {
                        if let Ok(c) = char::try_from(number) {
                            v.extend_from_slice(&text_bytes[start..ep]);
                            start = p + 1;
                            write_char_to_vec(c, &mut v);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        p += 1;
    }

    v.extend_from_slice(&text_bytes[start..p]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}
