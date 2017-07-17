use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

use error::Result;
use trit;
use trit::Trit;
use tryte::*;

const TERNARY_MIN: u16 = 0b11_11_11_11_11_11;
const TERNARY_NEG64: u16 = 0b00_11_01_11_00_11;
const TERNARY_NEG1: u16 = 0b00_00_00_00_00_11;
const TERNARY_0: u16 = 0b00_00_00_00_00_00;
const TERNARY_1: u16 = 0b00_00_00_00_00_01;
const TERNARY_64: u16 = 0b00_01_11_01_00_01;
const TERNARY_MAX: u16 = 0b01_01_01_01_01_01;

const TRYTE_MIN: Tryte = Tryte(TERNARY_MIN);
const TRYTE_NEG64: Tryte = Tryte(TERNARY_NEG64);
const TRYTE_NEG1: Tryte = Tryte(TERNARY_NEG1);
const TRYTE_0: Tryte = Tryte(TERNARY_0);
const TRYTE_1: Tryte = Tryte(TERNARY_1);
const TRYTE_64: Tryte = Tryte(TERNARY_64);
const TRYTE_MAX: Tryte = Tryte(TERNARY_MAX);

#[test]
fn tryte_into_i16() {
    assert_eq!(MIN_VALUE, TRYTE_MIN.into());
    assert_eq!(-64i16, TRYTE_NEG64.into());
    assert_eq!(-1i16, TRYTE_NEG1.into());
    assert_eq!(0i16, TRYTE_0.into());
    assert_eq!(1i16, TRYTE_1.into());
    assert_eq!(64i16, TRYTE_64.into());
    assert_eq!(MAX_VALUE, TRYTE_MAX.into());
}

#[test]
fn tryte_from_i16() {
    assert_eq!(Ok(Tryte(TERNARY_MIN)), Tryte::try_from(MIN_VALUE));
    assert_eq!(Ok(Tryte(TERNARY_NEG64)), Tryte::try_from(-64));
    assert_eq!(Ok(Tryte(TERNARY_NEG1)), Tryte::try_from(-1));
    assert_eq!(Ok(Tryte(TERNARY_0)), Tryte::try_from(0));
    assert_eq!(Ok(Tryte(TERNARY_1)), Tryte::try_from(1));
    assert_eq!(Ok(Tryte(TERNARY_64)), Tryte::try_from(64));
    assert_eq!(Ok(Tryte(TERNARY_MAX)), Tryte::try_from(MAX_VALUE));

    assert!(Tryte::try_from(MIN_VALUE - 1).is_err());
    assert!(Tryte::try_from(MAX_VALUE + 1).is_err());
}

#[test]
fn tryte_into_trit() {
    assert_eq!(Ok(trit::NEG), TRYTE_NEG1.try_into());
    assert_eq!(Ok(trit::ZERO), TRYTE_0.try_into());
    assert_eq!(Ok(trit::POS), TRYTE_1.try_into());

    assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_NEG64).is_err());
    assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_64).is_err());
}

#[test]
fn tryte_from_trit() {
    assert_eq!(TRYTE_NEG1, trit::NEG.into());
    assert_eq!(TRYTE_0, trit::ZERO.into());
    assert_eq!(TRYTE_1, trit::POS.into());
}

#[test]
fn tryte_from_bytes() {
    assert_eq!(Ok(TRYTE_MIN), from_bytes(0b11_11_11_11, 0b00_00_11_11));
    assert_eq!(Ok(TRYTE_NEG64), from_bytes(0b01_11_00_11, 0b00_00_00_11));
    assert_eq!(Ok(TRYTE_NEG1), from_bytes(0b00_00_00_11, 0b00_00_00_00));
    assert_eq!(Ok(TRYTE_0), from_bytes(0b00_00_00_00, 0b00_00_00_00));
    assert_eq!(Ok(TRYTE_1), from_bytes(0b00_00_00_01, 0b00_00_00_00));
    assert_eq!(Ok(TRYTE_64), from_bytes(0b11_01_00_01, 0b00_00_00_01));
    assert_eq!(Ok(TRYTE_MAX), from_bytes(0b01_01_01_01, 0b00_00_01_01));
}

fn from_bytes(low: u8, high: u8) -> Result<Tryte> {
    let mut cursor = Cursor::new(vec![low, high]);
    Ok(Tryte::from_bytes(&mut cursor)?)
}

#[test]
fn tryte_write_bytes() {
    assert_eq!(Ok((0b11_11_11_11, 0b00_00_11_11)), get_bytes(TRYTE_MIN));
    assert_eq!(Ok((0b01_11_00_11, 0b00_00_00_11)), get_bytes(TRYTE_NEG64));
    assert_eq!(Ok((0b00_00_00_11, 0b00_00_00_00)), get_bytes(TRYTE_NEG1));
    assert_eq!(Ok((0b00_00_00_00, 0b00_00_00_00)), get_bytes(TRYTE_0));
    assert_eq!(Ok((0b00_00_00_01, 0b00_00_00_00)), get_bytes(TRYTE_1));
    assert_eq!(Ok((0b11_01_00_01, 0b00_00_00_01)), get_bytes(TRYTE_64));
    assert_eq!(Ok((0b01_01_01_01, 0b00_00_01_01)), get_bytes(TRYTE_MAX));
}

fn get_bytes(tryte: Tryte) -> Result<(u8, u8)> {
    let mut bytes = vec![];
    tryte.write_bytes(&mut bytes)?;
    Ok((bytes[0], bytes[1]))
}

#[test]
fn tryte_negate() {
    let tryte_pos = Tryte::try_from(64).unwrap();
    let tryte_neg = Tryte::try_from(-64).unwrap();
    assert_eq!(tryte_pos, -tryte_neg);
}

#[test]
fn tryte_and() {
    assert_eq!(TRYTE_0, TRYTE_0 & TRYTE_0);
    assert_eq!(TRYTE_0, TRYTE_0 & TRYTE_MAX);
    assert_eq!(TRYTE_MIN, TRYTE_0 & TRYTE_MIN);
    assert_eq!(TRYTE_0, TRYTE_MAX & TRYTE_0);
    assert_eq!(TRYTE_MAX, TRYTE_MAX & TRYTE_MAX);
    assert_eq!(TRYTE_MIN, TRYTE_MAX & TRYTE_MIN);
    assert_eq!(TRYTE_MIN, TRYTE_MIN & TRYTE_0);
    assert_eq!(TRYTE_MIN, TRYTE_MIN & TRYTE_MAX);
    assert_eq!(TRYTE_MIN, TRYTE_MIN & TRYTE_MIN);
}

#[test]
fn tryte_or() {
    assert_eq!(TRYTE_0, TRYTE_0 | TRYTE_0);
    assert_eq!(TRYTE_MAX, TRYTE_0 | TRYTE_MAX);
    assert_eq!(TRYTE_0, TRYTE_0 | TRYTE_MIN);
    assert_eq!(TRYTE_MAX, TRYTE_MAX | TRYTE_0);
    assert_eq!(TRYTE_MAX, TRYTE_MAX | TRYTE_MAX);
    assert_eq!(TRYTE_MAX, TRYTE_MAX | TRYTE_MIN);
    assert_eq!(TRYTE_0, TRYTE_MIN | TRYTE_0);
    assert_eq!(TRYTE_MAX, TRYTE_MIN | TRYTE_MAX);
    assert_eq!(TRYTE_MIN, TRYTE_MIN | TRYTE_MIN);
}

#[test]
fn tryte_add() {
    let tryte_pos = Tryte::try_from(64).unwrap();
    let tryte_neg = Tryte::try_from(-64).unwrap();
    let tryte_sum = tryte_pos + tryte_neg;
    assert_eq!(0i16, tryte_sum.into());
}

#[test]
fn tryte_tcmp() {
    assert_eq!(TRYTE_MIN, TRYTE_MIN.tcmp(TRYTE_0));
    assert_eq!(TRYTE_MAX, TRYTE_MAX.tcmp(TRYTE_0));
    assert_eq!(TRYTE_NEG1, TRYTE_NEG1.tcmp(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_0.tcmp(TRYTE_0));
    assert_eq!(TRYTE_1, TRYTE_1.tcmp(TRYTE_0));
    assert_eq!(TRYTE_64, TRYTE_64.tcmp(TRYTE_0));
    assert_eq!(TRYTE_NEG64, TRYTE_NEG64.tcmp(TRYTE_0));

    assert_eq!(-TRYTE_MIN, TRYTE_0.tcmp(TRYTE_MIN));
    assert_eq!(-TRYTE_MAX, TRYTE_0.tcmp(TRYTE_MAX));
    assert_eq!(-TRYTE_NEG1, TRYTE_0.tcmp(TRYTE_NEG1));
    assert_eq!(-TRYTE_0, TRYTE_0.tcmp(TRYTE_0));
    assert_eq!(-TRYTE_1, TRYTE_0.tcmp(TRYTE_1));
    assert_eq!(-TRYTE_64, TRYTE_0.tcmp(TRYTE_64));
    assert_eq!(-TRYTE_NEG64, TRYTE_0.tcmp(TRYTE_NEG64));

    assert_eq!(TRYTE_0, TRYTE_MIN.tcmp(TRYTE_MIN));
    assert_eq!(TRYTE_0, TRYTE_MAX.tcmp(TRYTE_MAX));
    assert_eq!(TRYTE_0, TRYTE_NEG1.tcmp(TRYTE_NEG1));
    assert_eq!(TRYTE_0, TRYTE_0.tcmp(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_1.tcmp(TRYTE_1));
    assert_eq!(TRYTE_0, TRYTE_64.tcmp(TRYTE_64));
    assert_eq!(TRYTE_0, TRYTE_NEG64.tcmp(TRYTE_NEG64));
}

#[test]
fn tryte_tmul() {
    assert_eq!(TRYTE_0, TRYTE_MIN.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_MAX.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_NEG1.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_0.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_1.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_64.tmul(TRYTE_0));
    assert_eq!(TRYTE_0, TRYTE_NEG64.tmul(TRYTE_0));

    assert_eq!(TRYTE_MIN, TRYTE_MIN.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_MAX, TRYTE_MAX.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_NEG1, TRYTE_NEG1.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_0, TRYTE_0.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_1, TRYTE_1.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_64, TRYTE_64.tmul(TRYTE_MAX));
    assert_eq!(TRYTE_NEG64, TRYTE_NEG64.tmul(TRYTE_MAX));

    assert_eq!(-TRYTE_MIN, TRYTE_MIN.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_MAX, TRYTE_MAX.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_NEG1, TRYTE_NEG1.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_0, TRYTE_0.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_1, TRYTE_1.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_64, TRYTE_64.tmul(TRYTE_MIN));
    assert_eq!(-TRYTE_NEG64, TRYTE_NEG64.tmul(TRYTE_MIN));
}

#[test]
fn tryte_cmp() {
    assert!(TRYTE_0 == TRYTE_0);
    assert!(TRYTE_0 < TRYTE_MAX);
    assert!(TRYTE_0 > TRYTE_MIN);
    assert!(TRYTE_MAX > TRYTE_0);
    assert!(TRYTE_MAX > TRYTE_MIN);
    assert!(TRYTE_MAX == TRYTE_MAX);
    assert!(TRYTE_MIN < TRYTE_0);
    assert!(TRYTE_MIN < TRYTE_MAX);
    assert!(TRYTE_MIN == TRYTE_MIN);
}

#[test]
fn tryte_display_hytes() {
    assert_eq!("mm", format!("{}", TRYTE_MIN.display_hytes()));
    assert_eq!("bj", format!("{}", TRYTE_NEG64.display_hytes()));
    assert_eq!("0a", format!("{}", TRYTE_NEG1.display_hytes()));
    assert_eq!("00", format!("{}", TRYTE_0.display_hytes()));
    assert_eq!("0A", format!("{}", TRYTE_1.display_hytes()));
    assert_eq!("BJ", format!("{}", TRYTE_64.display_hytes()));
    assert_eq!("MM", format!("{}", TRYTE_MAX.display_hytes()));
}

#[test]
fn tryte_from_hyte_str() {
    assert_eq!(Ok(TRYTE_MIN), Tryte::from_hyte_str("mm"));
    assert_eq!(Ok(TRYTE_NEG64), Tryte::from_hyte_str("bj"));
    assert_eq!(Ok(TRYTE_NEG1), Tryte::from_hyte_str("0a"));
    assert_eq!(Ok(TRYTE_0), Tryte::from_hyte_str("00"));
    assert_eq!(Ok(TRYTE_1), Tryte::from_hyte_str("0A"));
    assert_eq!(Ok(TRYTE_64), Tryte::from_hyte_str("BJ"));
    assert_eq!(Ok(TRYTE_MAX), Tryte::from_hyte_str("MM"));

    assert!(Tryte::from_trit_str("").is_err());
    assert!(Tryte::from_trit_str("M").is_err());
    assert!(Tryte::from_trit_str("MMM").is_err());
    assert!(Tryte::from_trit_str("NN").is_err());
}

#[test]
fn tryte_display_trits() {
    assert_eq!("TTTTTT", format!("{}", TRYTE_MIN.display_trits()));
    assert_eq!("0T1T0T", format!("{}", TRYTE_NEG64.display_trits()));
    assert_eq!("00000T", format!("{}", TRYTE_NEG1.display_trits()));
    assert_eq!("000000", format!("{}", TRYTE_0.display_trits()));
    assert_eq!("000001", format!("{}", TRYTE_1.display_trits()));
    assert_eq!("01T101", format!("{}", TRYTE_64.display_trits()));
    assert_eq!("111111", format!("{}", TRYTE_MAX.display_trits()));
}

#[test]
fn tryte_from_trit_str() {
    assert_eq!(Ok(TRYTE_MIN), Tryte::from_trit_str("TTTTTT"));
    assert_eq!(Ok(TRYTE_NEG64), Tryte::from_trit_str("0T1T0T"));
    assert_eq!(Ok(TRYTE_NEG1), Tryte::from_trit_str("00000T"));
    assert_eq!(Ok(TRYTE_0), Tryte::from_trit_str("000000"));
    assert_eq!(Ok(TRYTE_1), Tryte::from_trit_str("000001"));
    assert_eq!(Ok(TRYTE_64), Tryte::from_trit_str("01T101"));
    assert_eq!(Ok(TRYTE_MAX), Tryte::from_trit_str("111111"));

    assert!(Tryte::from_trit_str("").is_err());
    assert!(Tryte::from_trit_str("TTTTT").is_err());
    assert!(Tryte::from_trit_str("TTTTTTT").is_err());
    assert!(Tryte::from_trit_str("222222").is_err());
}
