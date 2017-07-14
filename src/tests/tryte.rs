use std::convert::TryFrom;

use tryte::Tryte;

// 64 = 01T101 = 0b00_00_00_01_11_01_00_01
const TRYTE_64: u16 = 0b00_00_00_01_11_01_00_01;

#[test]
fn tryte_into_i16() {
    let tryte = Tryte(TRYTE_64);
    assert_eq!(64i16, tryte.into());
}

#[test]
fn tryte_from_i16() {
    let tryte = Tryte::try_from(64).unwrap();
    assert_eq!(TRYTE_64, tryte.0);
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
