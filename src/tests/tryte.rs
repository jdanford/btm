use std::convert::TryFrom;

use tryte::Tryte;

const TRYTE_NEG1: u16 = 0b00_00_00_00_00_00_00_11;
const TRYTE_0: u16 = 0b00_00_00_00_00_00_00_00;
const TRYTE_1: u16 = 0b00_00_00_00_00_00_00_01;
const TRYTE_64: u16 = 0b00_00_00_01_11_01_00_01;

#[test]
fn tryte_into_i16() {
    assert_eq!(-1i16, Tryte(TRYTE_NEG1).into());
    assert_eq!(0i16, Tryte(TRYTE_0).into());
    assert_eq!(1i16, Tryte(TRYTE_1).into());
    assert_eq!(64i16, Tryte(TRYTE_64).into());
}

#[test]
fn tryte_from_i16() {
    assert_eq!(TRYTE_NEG1, Tryte::try_from(-1).unwrap().0);
    assert_eq!(TRYTE_0, Tryte::try_from(0).unwrap().0);
    assert_eq!(TRYTE_1, Tryte::try_from(1).unwrap().0);
    assert_eq!(TRYTE_64, Tryte::try_from(64).unwrap().0);
}

#[test]
fn tryte_negate() {
    let tryte_pos = Tryte::try_from(64).unwrap();
    let tryte_neg = Tryte::try_from(-64).unwrap();
    assert_eq!(tryte_pos, -tryte_neg);
}

#[test]
fn tryte_add() {
    let tryte_pos = Tryte::try_from(64).unwrap();
    let tryte_neg = Tryte::try_from(-64).unwrap();
    let tryte_sum = tryte_pos + tryte_neg;
    assert_eq!(0i16, tryte_sum.into());
}

#[test]
fn tryte_display() {
    let tryte_pos = Tryte::try_from(64).unwrap();
    let tryte_neg = Tryte::try_from(-64).unwrap();
    let tryte_sum = tryte_pos + tryte_neg;
    assert_eq!(0i16, tryte_sum.into());
}
