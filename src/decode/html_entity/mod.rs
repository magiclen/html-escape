mod tables;

use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{convert::TryFrom, str::from_utf8_unchecked};
#[cfg(feature = "std")]
use std::io::{self, Write};

pub use tables::*;

use crate::functions::*;

#[derive(Clone, Copy)]
enum DecodedEntityValue {
    Named(&'static str),
    Character(char),
}

#[derive(Clone, Copy)]
struct DecodedEntity {
    start: usize,
    end:   usize,
    value: DecodedEntityValue,
}

#[inline]
fn decode_named_entity(name: &[u8]) -> Option<DecodedEntityValue> {
    NAMED_ENTITIES
        .binary_search_by(|(t_name, _)| t_name.cmp(&name))
        .ok()
        .map(|index| DecodedEntityValue::Named(NAMED_ENTITIES[index].1))
}

#[inline]
fn decode_decimal_entity(text: &str, start: usize, end: usize) -> Option<DecodedEntityValue> {
    let number = text[start..end].parse::<u32>().ok()?;
    let character = char::try_from(number).ok()?;

    Some(DecodedEntityValue::Character(character))
}

#[inline]
fn decode_hex_entity(text: &str, start: usize, end: usize) -> Option<DecodedEntityValue> {
    let number = u32::from_str_radix(&text[start..end], 16).ok()?;
    let character = char::try_from(number).ok()?;

    Some(DecodedEntityValue::Character(character))
}

fn find_decoded_entity(text: &str, start: usize) -> Option<DecodedEntity> {
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let mut p = start;

    'search: while p < text_length {
        if text_bytes[p] != b'&' {
            p += 1;

            continue;
        }

        let entity_start = p;

        p += 1;

        if p == text_length {
            return None;
        }

        match text_bytes[p] {
            b'&' => continue 'search,
            b';' => {
                p += 1;
            },
            b'#' => {
                p += 1;

                if p == text_length {
                    return None;
                }

                match text_bytes[p] {
                    b'&' => continue 'search,
                    b';' => {
                        p += 1;
                    },
                    b'x' | b'X' => {
                        p += 1;

                        if p == text_length {
                            return None;
                        }

                        match text_bytes[p] {
                            b'&' => continue 'search,
                            b';' => {
                                p += 1;
                            },
                            _ => {
                                let hex_start = p;

                                loop {
                                    p += 1;

                                    if p == text_length {
                                        return None;
                                    }

                                    match text_bytes[p] {
                                        b'&' => continue 'search,
                                        b';' => {
                                            if let Some(value) =
                                                decode_hex_entity(text, hex_start, p)
                                            {
                                                return Some(DecodedEntity {
                                                    start: entity_start,
                                                    end: p + 1,
                                                    value,
                                                });
                                            }

                                            p += 1;

                                            continue 'search;
                                        },
                                        _ => (),
                                    }
                                }
                            },
                        }
                    },
                    _ => {
                        let number_start = p;

                        loop {
                            p += 1;

                            if p == text_length {
                                return None;
                            }

                            match text_bytes[p] {
                                b'&' => continue 'search,
                                b';' => {
                                    if let Some(value) =
                                        decode_decimal_entity(text, number_start, p)
                                    {
                                        return Some(DecodedEntity {
                                            start: entity_start,
                                            end: p + 1,
                                            value,
                                        });
                                    }

                                    p += 1;

                                    continue 'search;
                                },
                                _ => (),
                            }
                        }
                    },
                }
            },
            _ => {
                let name_start = p;

                loop {
                    p += 1;

                    if p == text_length {
                        return None;
                    }

                    match text_bytes[p] {
                        b'&' => continue 'search,
                        b';' => {
                            if let Some(value) = decode_named_entity(&text_bytes[name_start..p]) {
                                return Some(DecodedEntity {
                                    start: entity_start,
                                    end: p + 1,
                                    value,
                                });
                            }

                            p += 1;

                            continue 'search;
                        },
                        _ => (),
                    }
                }
            },
        }
    }

    None
}

#[inline]
fn write_decoded_entity_to_vec(value: DecodedEntityValue, output: &mut Vec<u8>) {
    match value {
        DecodedEntityValue::Named(entity) => output.extend_from_slice(entity.as_bytes()),
        DecodedEntityValue::Character(character) => write_char_to_vec(character, output),
    }
}

#[cfg(feature = "std")]
#[inline]
fn write_decoded_entity_to_writer<W: Write>(
    value: DecodedEntityValue,
    output: &mut W,
) -> Result<(), io::Error> {
    match value {
        DecodedEntityValue::Named(entity) => output.write_all(entity.as_bytes()),
        DecodedEntityValue::Character(character) => write_char_to_writer(character, output),
    }
}

/// Decode html entities in a given string.
pub fn decode_html_entities<S: ?Sized + AsRef<str>>(text: &S) -> Cow<'_, str> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    let entity = match find_decoded_entity(text, 0) {
        Some(entity) => entity,
        None => return Cow::from(text),
    };

    let mut v = Vec::with_capacity(text_length);
    let mut start = entity.end;

    v.extend_from_slice(&text_bytes[..entity.start]);
    write_decoded_entity_to_vec(entity.value, &mut v);

    while let Some(entity) = find_decoded_entity(text, start) {
        v.extend_from_slice(&text_bytes[start..entity.start]);
        write_decoded_entity_to_vec(entity.value, &mut v);
        start = entity.end;
    }

    v.extend_from_slice(&text_bytes[start..]);

    Cow::from(unsafe { String::from_utf8_unchecked(v) })
}

/// Decode html entities in a given string to a mutable `String` reference and return the decoded string slice.
pub fn decode_html_entities_to_string<S: AsRef<str>>(text: S, output: &mut String) -> &str {
    unsafe { from_utf8_unchecked(decode_html_entities_to_vec(text, output.as_mut_vec())) }
}

/// Decode html entities in a given string to a mutable `Vec<u8>` reference and return the decoded data slice.
pub fn decode_html_entities_to_vec<S: AsRef<str>>(text: S, output: &mut Vec<u8>) -> &[u8] {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();
    let text_length = text_bytes.len();

    output.reserve(text_length);

    let current_length = output.len();

    let mut start = 0;

    while let Some(entity) = find_decoded_entity(text, start) {
        output.extend_from_slice(&text_bytes[start..entity.start]);
        write_decoded_entity_to_vec(entity.value, output);
        start = entity.end;
    }

    output.extend_from_slice(&text_bytes[start..]);

    &output[current_length..]
}

#[cfg(feature = "std")]
/// Decode html entities in a given string to a writer.
pub fn decode_html_entities_to_writer<S: AsRef<str>, W: Write>(
    text: S,
    output: &mut W,
) -> Result<(), io::Error> {
    let text = text.as_ref();
    let text_bytes = text.as_bytes();

    let mut start = 0;

    while let Some(entity) = find_decoded_entity(text, start) {
        output.write_all(&text_bytes[start..entity.start])?;
        write_decoded_entity_to_writer(entity.value, output)?;
        start = entity.end;
    }

    output.write_all(&text_bytes[start..])
}
