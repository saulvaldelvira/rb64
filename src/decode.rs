use core::str::Chars;

use super::Result;
use crate::prelude::{Box, Vec};

use crate::unreachable;

#[inline(always)]
fn next(chars: &mut Chars<'_>) -> Result<Option<i8>> {
    let Some(c) = chars.next() else {
        return Ok(None);
    };
    let mut n = c as i8;
    n -= match c {
            'A'..='Z' => 'A' as i8,
            'a'..='z' => 'a' as i8 - 26,
            '0'..='9' => '0' as i8 - 52,
            '+' => '+' as i8 - 62,
            '/' => '/' as i8 - 63,
            '=' => return Ok(None),
            '\r' | '\n' => return next(chars),
            _ => return Err(format!("Unknown character to decode: '{c}'").into()),
        };
    Ok(Some(n))
}

/// Decode a Base64-encoded string
///
/// This function returns a [`Box<[u8]>`] with the content of
/// the given string.
///
/// The text does not need to be padded with '='
///
/// # Example
/// ```
/// use rb64::decode;
/// use std::str;
///
/// let bytes = decode("SGVsbG8gd29ybGQh").unwrap();
/// let msg = str::from_utf8(&bytes).unwrap();
/// assert_eq!(msg, "Hello world!");
/// ```
pub fn decode(text: &str) -> Result<Box<[u8]>> {
    let len = text.len();
    if !len.is_multiple_of(4) {
        return Err("Base64 string length must be multiple of 4".into())
    }
    let n_padding = text.chars().rev().take(2).filter(|&c| c == '=').count();
    let capacity = (3 * (len / 4)) - n_padding;

    let mut decoded = Vec::<u8>::with_capacity(capacity);
    macro_rules! push {
        ($e:expr) => {
            if decoded.len() >= decoded.capacity() {
                unreachable!("The capacity will always be enough");
            }
            decoded.push((($e) & 0b11111111) as u8);
        };
    }

    let mut chars = text.chars();
    'main: loop {
        let mut n = 0_u32;

        let mut offset = 4;
        for _ in 0..2 {
            for i in 0..2 {
                n <<= 6;
                match next(&mut chars)? {
                    Some(c) => n |= c as u32,
                    None => {
                        if i == 1 {
                            push!(n >> offset);
                        }
                        break 'main;
                    }
                }
            }
            push!(n >> offset);
            offset *= 2;
        }
        push!(n);
    }

    Ok(decoded.into_boxed_slice())
}
