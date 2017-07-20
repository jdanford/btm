use std::convert::TryFrom;

use registers;
use registers::{StandardRegister, SystemRegister};

#[test]
fn standard_register_from_trit4() {
    assert_eq!(
        Ok(registers::ZERO),
        StandardRegister::try_from(0b00_00_00_00)
    );
    assert_eq!(Ok(registers::LO), StandardRegister::try_from(0b00_00_00_01));
    assert_eq!(Ok(registers::HI), StandardRegister::try_from(0b00_00_01_11));
    assert_eq!(Ok(registers::SP), StandardRegister::try_from(0b00_00_01_00));
    assert_eq!(Ok(registers::FP), StandardRegister::try_from(0b00_00_01_01));
    assert_eq!(Ok(registers::RA), StandardRegister::try_from(0b00_01_11_11));
    assert_eq!(Ok(registers::A0), StandardRegister::try_from(0b00_01_11_00));
    assert_eq!(Ok(registers::A1), StandardRegister::try_from(0b00_01_11_01));
    assert_eq!(Ok(registers::A2), StandardRegister::try_from(0b00_01_00_11));
    assert_eq!(Ok(registers::A3), StandardRegister::try_from(0b00_01_00_00));
    assert_eq!(Ok(registers::A4), StandardRegister::try_from(0b00_01_00_01));
    assert_eq!(Ok(registers::A5), StandardRegister::try_from(0b00_01_01_11));
    assert_eq!(Ok(registers::S0), StandardRegister::try_from(0b00_01_01_00));
    assert_eq!(Ok(registers::S1), StandardRegister::try_from(0b00_01_01_01));
    assert_eq!(Ok(registers::S2), StandardRegister::try_from(0b01_11_11_11));
    assert_eq!(Ok(registers::S3), StandardRegister::try_from(0b01_11_11_00));
    assert_eq!(Ok(registers::S4), StandardRegister::try_from(0b01_11_11_01));
    assert_eq!(Ok(registers::S5), StandardRegister::try_from(0b01_11_00_11));
    assert_eq!(Ok(registers::T0), StandardRegister::try_from(0b01_11_00_00));
    assert_eq!(Ok(registers::T1), StandardRegister::try_from(0b01_11_00_01));
    assert_eq!(Ok(registers::T2), StandardRegister::try_from(0b01_11_01_11));
    assert_eq!(Ok(registers::T3), StandardRegister::try_from(0b01_11_01_00));
    assert_eq!(Ok(registers::T4), StandardRegister::try_from(0b01_11_01_01));
    assert_eq!(Ok(registers::T5), StandardRegister::try_from(0b01_00_11_11));

    assert!(StandardRegister::try_from(0b00_00_00_11).is_err());
    assert!(StandardRegister::try_from(0b01_00_11_00).is_err());
}

#[test]
fn system_register_from_trit4() {
    assert_eq!(Ok(registers::EHA), SystemRegister::try_from(0b00_00_00_00));
    assert_eq!(Ok(registers::ERA), SystemRegister::try_from(0b00_00_00_01));
    assert_eq!(Ok(registers::EC), SystemRegister::try_from(0b00_00_01_11));
    assert_eq!(Ok(registers::ED), SystemRegister::try_from(0b00_00_01_00));

    assert!(SystemRegister::try_from(0b00_00_00_11).is_err());
    assert!(SystemRegister::try_from(0b00_00_01_01).is_err());
}
