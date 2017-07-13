use std::convert::TryFrom;

use trit::Trit;
use tryte::Tryte;

#[test]
fn tryte_into_i16() {
    let tryte = Tryte(Trit::BITS_POS);
    assert_eq!(1i16, tryte.into());
}

#[test]
fn tryte_from_i16() {
    let tryte = Tryte::try_from(1).unwrap();
    assert_eq!(Trit::BITS_POS, tryte.0);
}

#[test]
fn tryte_negate() {
    let tryte_pos = Tryte::try_from(1).unwrap();
    let tryte_neg = Tryte::try_from(-1).unwrap();
    assert_eq!(tryte_pos, -tryte_neg);
}

#[test]
fn tryte_add() {
    let tryte_pos = Tryte::try_from(1).unwrap();
    let tryte_neg = Tryte::try_from(-1).unwrap();
    let tryte_sum = tryte_pos + tryte_neg;
    assert_eq!(0i16, tryte_sum.into());
}
