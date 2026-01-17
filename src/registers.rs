use std::ops::{Index, IndexMut, RangeInclusive};

use ternary::{T24, Tryte, tables::TRIT4_TO_I8, tryte};

use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Register(i8);

pub const ZERO: Register = Register(0);
pub const LO: Register = Register(1);
pub const HI: Register = Register(2);
pub const SP: Register = Register(3);
pub const FP: Register = Register(4);
pub const RA: Register = Register(5);
pub const A0: Register = Register(6);
pub const A1: Register = Register(7);
pub const A2: Register = Register(8);
pub const A3: Register = Register(9);
pub const A4: Register = Register(10);
pub const A5: Register = Register(11);
pub const S0: Register = Register(12);
pub const S1: Register = Register(13);
pub const S2: Register = Register(14);
pub const S3: Register = Register(15);
pub const S4: Register = Register(16);
pub const S5: Register = Register(17);
pub const T0: Register = Register(18);
pub const T1: Register = Register(19);
pub const T2: Register = Register(20);
pub const T3: Register = Register(21);
pub const T4: Register = Register(22);
pub const T5: Register = Register(23);

const VALID_REGISTER_RANGE: RangeInclusive<i8> = ZERO.0..=T5.0;
#[allow(clippy::cast_sign_loss)]
const REGISTER_COUNT: usize =
    (*VALID_REGISTER_RANGE.end() - *VALID_REGISTER_RANGE.start()) as usize;

impl Register {
    #[allow(clippy::cast_sign_loss)]
    pub fn from_trit4(trit4: u8) -> Result<Self> {
        let index = TRIT4_TO_I8[trit4 as usize];

        if !VALID_REGISTER_RANGE.contains(&index) {
            return Err(Error::InvalidRegister(index));
        }

        Ok(Self(index))
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn into_index(self) -> usize {
        (self.0 - VALID_REGISTER_RANGE.start()) as usize
    }
}

pub struct Registers([T24; REGISTER_COUNT]);

impl Registers {
    pub fn new() -> Self {
        Self([T24::ZERO; REGISTER_COUNT])
    }
}

impl Index<Register> for Registers {
    type Output = T24;

    fn index(&self, register: Register) -> &Self::Output {
        &self.0[register.into_index()]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self.0[register.into_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_from_trit4() {
        assert_eq!(ZERO, Register::from_trit4(0b00_00_00_00).unwrap());
        assert_eq!(LO, Register::from_trit4(0b00_00_00_01).unwrap());
        assert_eq!(HI, Register::from_trit4(0b00_00_01_11).unwrap());
        assert_eq!(SP, Register::from_trit4(0b00_00_01_00).unwrap());
        assert_eq!(FP, Register::from_trit4(0b00_00_01_01).unwrap());
        assert_eq!(RA, Register::from_trit4(0b00_01_11_11).unwrap());
        assert_eq!(A0, Register::from_trit4(0b00_01_11_00).unwrap());
        assert_eq!(A1, Register::from_trit4(0b00_01_11_01).unwrap());
        assert_eq!(A2, Register::from_trit4(0b00_01_00_11).unwrap());
        assert_eq!(A3, Register::from_trit4(0b00_01_00_00).unwrap());
        assert_eq!(A4, Register::from_trit4(0b00_01_00_01).unwrap());
        assert_eq!(A5, Register::from_trit4(0b00_01_01_11).unwrap());
        assert_eq!(S0, Register::from_trit4(0b00_01_01_00).unwrap());
        assert_eq!(S1, Register::from_trit4(0b00_01_01_01).unwrap());
        assert_eq!(S2, Register::from_trit4(0b01_11_11_11).unwrap());
        assert_eq!(S3, Register::from_trit4(0b01_11_11_00).unwrap());
        assert_eq!(S4, Register::from_trit4(0b01_11_11_01).unwrap());
        assert_eq!(S5, Register::from_trit4(0b01_11_00_11).unwrap());
        assert_eq!(T0, Register::from_trit4(0b01_11_00_00).unwrap());
        assert_eq!(T1, Register::from_trit4(0b01_11_00_01).unwrap());
        assert_eq!(T2, Register::from_trit4(0b01_11_01_11).unwrap());
        assert_eq!(T3, Register::from_trit4(0b01_11_01_00).unwrap());
        assert_eq!(T4, Register::from_trit4(0b01_11_01_01).unwrap());
        assert_eq!(T5, Register::from_trit4(0b01_00_11_11).unwrap());

        assert!(Register::from_trit4(0b00_00_00_11).is_err());
        assert!(Register::from_trit4(0b01_00_11_00).is_err());
    }
}
