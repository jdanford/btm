use std::convert::TryInto;
use std::io::Cursor;

use ternary::Result;
use ternary::trit;
use ternary::Trit;
use ternary::tryte::*;

use tests::constants::*;

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

    assert!(from_bytes(0b01_01_10_01, 0b00_00_01_01).is_err());
}

fn from_bytes(low: u8, high: u8) -> Result<Tryte> {
    let mut cursor = Cursor::new(vec![low, high]);
    Ok(Tryte::from_bytes(&mut cursor)?)
}

#[test]
fn tryte_write_bytes() {
    assert_eq!(vec![0b11_11_11_11, 0b00_00_11_11], get_bytes(TRYTE_MIN));
    assert_eq!(vec![0b01_11_00_11, 0b00_00_00_11], get_bytes(TRYTE_NEG64));
    assert_eq!(vec![0b00_00_00_11, 0b00_00_00_00], get_bytes(TRYTE_NEG1));
    assert_eq!(vec![0b00_00_00_00, 0b00_00_00_00], get_bytes(TRYTE_0));
    assert_eq!(vec![0b00_00_00_01, 0b00_00_00_00], get_bytes(TRYTE_1));
    assert_eq!(vec![0b11_01_00_01, 0b00_00_00_01], get_bytes(TRYTE_64));
    assert_eq!(vec![0b01_01_01_01, 0b00_00_01_01], get_bytes(TRYTE_MAX));
}

fn get_bytes(tryte: Tryte) -> Vec<u8> {
    let mut bytes = vec![];
    tryte.write_bytes(&mut bytes).unwrap();
    bytes
}

#[test]
fn tryte_display_hytes() {
    assert_eq!("mm", format!("{}", TRYTE_MIN));
    assert_eq!("bj", format!("{}", TRYTE_NEG64));
    assert_eq!("0a", format!("{}", TRYTE_NEG1));
    assert_eq!("00", format!("{}", TRYTE_0));
    assert_eq!("0A", format!("{}", TRYTE_1));
    assert_eq!("BJ", format!("{}", TRYTE_64));
    assert_eq!("MM", format!("{}", TRYTE_MAX));
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

    assert!(Tryte::from_hyte_str("").is_err());
    assert!(Tryte::from_hyte_str("M").is_err());
    assert!(Tryte::from_hyte_str("MMM").is_err());
    assert!(Tryte::from_hyte_str("NN").is_err());
}

#[test]
fn tryte_negate() {
    assert_eq!(TRYTE_MIN, -TRYTE_MAX);
    assert_eq!(TRYTE_NEG64, -TRYTE_64);
    assert_eq!(TRYTE_NEG1, -TRYTE_1);
    assert_eq!(TRYTE_0, -TRYTE_0);
    assert_eq!(TRYTE_1, -TRYTE_NEG1);
    assert_eq!(TRYTE_64, -TRYTE_NEG64);
    assert_eq!(TRYTE_MAX, -TRYTE_MIN);

    assert_eq!(TRYTE_MAX, -TRYTE_MIN);
    assert_eq!(TRYTE_64, -TRYTE_NEG64);
    assert_eq!(TRYTE_1, -TRYTE_NEG1);
    assert_eq!(TRYTE_0, -TRYTE_0);
    assert_eq!(TRYTE_NEG1, -TRYTE_1);
    assert_eq!(TRYTE_NEG64, -TRYTE_64);
    assert_eq!(TRYTE_MIN, -TRYTE_MAX);
}

#[test]
fn tryte_add() {
    assert_eq!(
        (TRYTE_0, trit::ZERO),
        TRYTE_1.add_with_carry(TRYTE_NEG1, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_0, trit::ZERO),
        TRYTE_64.add_with_carry(TRYTE_NEG64, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_0, trit::ZERO),
        TRYTE_MAX.add_with_carry(TRYTE_MIN, trit::ZERO)
    );

    assert_eq!(
        (TRYTE_MIN, trit::ZERO),
        TRYTE_MIN.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_NEG64, trit::ZERO),
        TRYTE_NEG64.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_NEG1, trit::ZERO),
        TRYTE_NEG1.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_0, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_1, trit::ZERO),
        TRYTE_1.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_64, trit::ZERO),
        TRYTE_64.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_MAX, trit::ZERO),
        TRYTE_MAX.add_with_carry(TRYTE_0, trit::ZERO)
    );

    assert_eq!(
        (TRYTE_MIN, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_MIN, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_NEG64, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_NEG64, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_NEG1, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_NEG1, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_0, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_0, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_1, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_1, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_64, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_64, trit::ZERO)
    );
    assert_eq!(
        (TRYTE_MAX, trit::ZERO),
        TRYTE_0.add_with_carry(TRYTE_MAX, trit::ZERO)
    );
}
