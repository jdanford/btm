use std::char;

use super::constants::*;
use super::error::{Error, Result};
use super::trit;
use super::tryte;
use super::tryte::Tryte;
use super::Ternary;

const SINGLE_RANGE: usize = 243;
const DOUBLE_RANGE: usize = 19_683;
const TRIPLE_RANGE: usize = 1_594_323;

const SINGLE_OFFSET: isize = (SINGLE_RANGE as isize - 1) / 2;
const DOUBLE_OFFSET: isize = (DOUBLE_RANGE as isize - 1) / 2;
const TRIPLE_OFFSET: isize = (TRIPLE_RANGE as isize - 1) / 2;

const SINGLE_MIN: usize = 0;
const SINGLE_MAX: usize = SINGLE_MIN + SINGLE_RANGE - 1;

const DOUBLE_MIN: usize = SINGLE_MAX + 1;
const DOUBLE_MAX: usize = DOUBLE_MIN + DOUBLE_RANGE - 1;

const TRIPLE_MIN: usize = DOUBLE_MAX + 1;
const TRIPLE_MAX: usize = TRIPLE_MIN + TRIPLE_RANGE - 1;

static SINGLE_START_BITMASK: u16 = 0b00_11_11_11_11_11;

static DOUBLE_START_BITMASK: u16 = 0b00_00_11_11_11_11;
static DOUBLE_START_PATTERN: u16 = 0b01_00_00_00_00_00;

static TRIPLE_START_BITMASK: u16 = 0b00_00_00_11_11_11;
static TRIPLE_START_PATTERN: u16 = 0b01_01_00_00_00_00;

static CONTINUATION_BITMASK: u16 = 0b00_11_11_11_11_11;
static CONTINUATION_PATTERN: u16 = 0b11_00_00_00_00_00;

pub fn encode_str(dest: &mut [Tryte], s: &str) -> Result<usize> {
    let offset = WORD_LEN;
    let mut i = 0;
    for c in s.chars() {
        let start = i + offset;
        let end = start + 3;
        let mut slice = &mut dest[start..end];
        i += encode_char(slice, c)?;
    }

    dest[0..offset].read_i64(i as i64)?;
    Ok(i)
}

pub fn decode_str(src: &[Tryte]) -> Result<(String, usize)> {
    let offset = WORD_LEN;
    let len = src[0..offset].into_i64() as usize;
    let mut s = String::new();

    let mut i = 0;
    while i < len {
        let start = i + offset;
        let end = start + 3;
        let slice = &src[start..end];
        let (c, j) = decode_char(slice)?;
        s.push(c);
        i += j;
    }

    Ok((s, i))
}

pub fn encode_char(dest: &mut [Tryte], c: char) -> Result<usize> {
    let codepoint = c as u32;
    let (len, codepoint_offset) = match codepoint as usize {
        SINGLE_MIN...SINGLE_MAX => Ok((1, SINGLE_OFFSET)),
        DOUBLE_MIN...DOUBLE_MAX => Ok((2, DOUBLE_OFFSET)),
        TRIPLE_MIN...TRIPLE_MAX => Ok((3, TRIPLE_OFFSET)),
        _ => Err(Error::InvalidCharacter(c)),
    }?;

    let src = {
        let mut tmp = [tryte::ZERO; WORD_LEN];
        let shifted_codepoint = shift_codepoint(codepoint, codepoint_offset);
        tmp.read_i64(shifted_codepoint as i64)?;
        tmp
    };

    match len {
        1 => {
            dest[0] = src[0];
        }

        2 => {
            let src_0 = src[0].0;
            let src_1 = src[1].0;

            let double_start_trits = DOUBLE_START_PATTERN | (src_0 & DOUBLE_START_BITMASK);
            dest[0] = Tryte(double_start_trits);

            let continuation_trits = CONTINUATION_PATTERN |
                (src_0 >> 8 | src_1 << 4 & CONTINUATION_BITMASK);
            dest[1] = Tryte(continuation_trits);
        }

        3 => {
            let src_0 = src[0].0;
            let src_1 = src[1].0;
            let src_2 = src[2].0;

            let triple_start_trits = TRIPLE_START_PATTERN | (src_0 & TRIPLE_START_BITMASK);
            dest[0] = Tryte(triple_start_trits);

            let continuation1_trits = CONTINUATION_PATTERN |
                (src_0 >> 6 | src_1 << 6 & CONTINUATION_BITMASK);
            dest[1] = Tryte(continuation1_trits);

            let continuation2_trits = CONTINUATION_PATTERN |
                (src_1 >> 4 | src_2 << 8 & CONTINUATION_BITMASK);
            dest[2] = Tryte(continuation2_trits);
        }

        _ => unreachable!(),
    }

    Ok(len)
}

pub fn decode_char(src: &[Tryte]) -> Result<(char, usize)> {
    let mut dest = [tryte::ZERO; WORD_LEN];

    let high_trit = src.get_trit(5);
    let next_high_trit = src.get_trit(4);
    let (codepoint_offset, len) = match (high_trit, next_high_trit) {
        (trit::ZERO, _) => {
            dest[0] = src[0];
            Ok((SINGLE_OFFSET, 1))
        }

        (trit::POS, trit::ZERO) => {
            if src.get_trit(11) != trit::NEG {
                return Err(invalid_encoding_from_trytes(&src[1..2]));
            }

            let double_start_trits = src[0].0 & DOUBLE_START_BITMASK;
            let continuation_trits = src[1].0 & CONTINUATION_BITMASK;

            let dest0_trits = double_start_trits | (continuation_trits << 8 & tryte::BITMASK);
            dest[0] = Tryte(dest0_trits);

            let dest1_trits = continuation_trits >> 4;
            dest[1] = Tryte(dest1_trits);

            Ok((DOUBLE_OFFSET, 2))
        }

        (trit::POS, trit::POS) => {
            if src.get_trit(11) != trit::NEG {
                return Err(invalid_encoding_from_trytes(&src[1..2]));
            }

            if src.get_trit(17) != trit::NEG {
                return Err(invalid_encoding_from_trytes(&src[2..3]));
            }

            let triple_start_trits = src[0].0 & TRIPLE_START_BITMASK;
            let continuation1_trits = src[1].0 & CONTINUATION_BITMASK;
            let continuation2_trits = src[2].0 & CONTINUATION_BITMASK;

            let dest0_trits = triple_start_trits | (continuation1_trits << 6 & tryte::BITMASK);
            dest[0] = Tryte(dest0_trits);

            let dest1_trits = continuation1_trits >> 6 |
                (continuation2_trits << 4 & tryte::BITMASK);
            dest[1] = Tryte(dest1_trits);

            let dest2_trits = continuation2_trits >> 8;
            dest[2] = Tryte(dest2_trits);

            Ok((TRIPLE_OFFSET, 3))
        }

        _ => Err(invalid_encoding_from_trytes(&src[0..1])),
    }?;

    let shifted_codepoint = dest.into_i64() as i32;
    let codepoint = unshift_codepoint(shifted_codepoint, codepoint_offset);
    let c = char::from_u32(codepoint).ok_or_else(|| {
        invalid_encoding_from_trytes(src)
    })?;

    Ok((c, len))
}

pub fn invalid_encoding_from_trytes(src: &[Tryte]) -> Error {
    let mut bytes = Vec::new();
    src.write_trits(&mut bytes).unwrap();
    let s = String::from_utf8_lossy(&bytes).into_owned();
    Error::InvalidEncoding(s)
}

fn shift_codepoint(codepoint: u32, offset: isize) -> i32 {
    (codepoint as i32).wrapping_sub(offset as i32)
}

fn unshift_codepoint(shifted_codepoint: i32, offset: isize) -> u32 {
    shifted_codepoint.wrapping_add(offset as i32) as u32
}
