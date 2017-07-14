use std::convert::{TryFrom, TryInto};

use trit;
use trit::Trit;
use tryte::*;

const TERNARY_MIN: u16 = 0b00_00_11_11_11_11_11_11;
const TERNARY_MAX: u16 = 0b00_00_01_01_01_01_01_01;
const TERNARY_NEG1: u16 = 0b00_00_00_00_00_00_00_11;
const TERNARY_0: u16 = 0b00_00_00_00_00_00_00_00;
const TERNARY_1: u16 = 0b00_00_00_00_00_00_00_01;
const TERNARY_64: u16 = 0b00_00_00_01_11_01_00_01;
const TERNARY_NEG64: u16 = 0b00_00_00_11_01_11_00_11;

const TRYTE_MIN: Tryte = Tryte(TERNARY_MIN);
const TRYTE_MAX: Tryte = Tryte(TERNARY_MAX);
const TRYTE_NEG1: Tryte = Tryte(TERNARY_NEG1);
const TRYTE_0: Tryte = Tryte(TERNARY_0);
const TRYTE_1: Tryte = Tryte(TERNARY_1);
const TRYTE_64: Tryte = Tryte(TERNARY_64);
const TRYTE_NEG64: Tryte = Tryte(TERNARY_NEG64);

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
    assert!(Tryte::try_from(MIN_VALUE - 1).is_err());
    assert!(Tryte::try_from(MAX_VALUE + 1).is_err());
    assert_eq!(TERNARY_MIN, Tryte::try_from(MIN_VALUE).unwrap().0);
    assert_eq!(TERNARY_NEG1, Tryte::try_from(-1).unwrap().0);
    assert_eq!(TERNARY_NEG64, Tryte::try_from(-64).unwrap().0);
    assert_eq!(TERNARY_0, Tryte::try_from(0).unwrap().0);
    assert_eq!(TERNARY_1, Tryte::try_from(1).unwrap().0);
    assert_eq!(TERNARY_64, Tryte::try_from(64).unwrap().0);
    assert_eq!(TERNARY_MAX, Tryte::try_from(MAX_VALUE).unwrap().0);
}

#[test]
fn tryte_into_trit() {
    assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_NEG64).is_err());
    assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_64).is_err());
    assert_eq!(trit::NEG, TRYTE_NEG1.try_into().unwrap());
    assert_eq!(trit::ZERO, TRYTE_0.try_into().unwrap());
    assert_eq!(trit::POS, TRYTE_1.try_into().unwrap());
}

#[test]
fn tryte_from_trit() {
    assert_eq!(TRYTE_NEG1, trit::NEG.into());
    assert_eq!(TRYTE_0, trit::ZERO.into());
    assert_eq!(TRYTE_1, trit::POS.into());
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
