use std::ops::{Index, IndexMut};

use ternary::{T24, Tryte, tables::TRIT4_TO_I8, tryte};

use crate::error::{Error, Result};

pub trait Register: Sized {
    const COUNT: usize;

    fn from_trit4(trit4: u8) -> Result<Self> {
        let i = TRIT4_TO_I8[trit4 as usize] as u8;
        if i as usize >= Self::COUNT {
            return Err(Error::InvalidRegister(i));
        }

        Ok(Self::from_index(i as usize))
    }

    fn from_index(i: usize) -> Self;
    fn into_index(self) -> usize;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StandardRegister(usize);

pub const ZERO: StandardRegister = StandardRegister(0);
pub const LO: StandardRegister = StandardRegister(1);
pub const HI: StandardRegister = StandardRegister(2);
pub const SP: StandardRegister = StandardRegister(3);
pub const FP: StandardRegister = StandardRegister(4);
pub const RA: StandardRegister = StandardRegister(5);
pub const A0: StandardRegister = StandardRegister(6);
pub const A1: StandardRegister = StandardRegister(7);
pub const A2: StandardRegister = StandardRegister(8);
pub const A3: StandardRegister = StandardRegister(9);
pub const A4: StandardRegister = StandardRegister(10);
pub const A5: StandardRegister = StandardRegister(11);
pub const S0: StandardRegister = StandardRegister(12);
pub const S1: StandardRegister = StandardRegister(13);
pub const S2: StandardRegister = StandardRegister(14);
pub const S3: StandardRegister = StandardRegister(15);
pub const S4: StandardRegister = StandardRegister(16);
pub const S5: StandardRegister = StandardRegister(17);
pub const T0: StandardRegister = StandardRegister(18);
pub const T1: StandardRegister = StandardRegister(19);
pub const T2: StandardRegister = StandardRegister(20);
pub const T3: StandardRegister = StandardRegister(21);
pub const T4: StandardRegister = StandardRegister(22);
pub const T5: StandardRegister = StandardRegister(23);

impl Register for StandardRegister {
    const COUNT: usize = 24;

    fn from_index(i: usize) -> Self {
        StandardRegister(i)
    }

    fn into_index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SystemRegister(usize);

pub const EHA: SystemRegister = SystemRegister(0);
pub const ERA: SystemRegister = SystemRegister(1);
pub const EC: SystemRegister = SystemRegister(2);
pub const ED: SystemRegister = SystemRegister(3);

impl Register for SystemRegister {
    const COUNT: usize = 4;

    fn from_index(i: usize) -> Self {
        SystemRegister(i)
    }

    fn into_index(self) -> usize {
        self.0 + StandardRegister::COUNT
    }
}

const TOTAL_COUNT: usize = StandardRegister::COUNT + SystemRegister::COUNT;

pub struct RegisterFile {
    registers: [T24; TOTAL_COUNT],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [T24::ZERO; TOTAL_COUNT],
        }
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Register> Index<R> for RegisterFile {
    type Output = T24;

    fn index(&self, register: R) -> &Self::Output {
        let i = register.into_index();
        &self.registers[i]
    }
}

impl<R: Register> IndexMut<R> for RegisterFile {
    fn index_mut(&mut self, register: R) -> &mut Self::Output {
        let i = register.into_index();
        &mut self.registers[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_register_from_trit4() {
        assert_eq!(ZERO, StandardRegister::from_trit4(0b00_00_00_00).unwrap());
        assert_eq!(LO, StandardRegister::from_trit4(0b00_00_00_01).unwrap());
        assert_eq!(HI, StandardRegister::from_trit4(0b00_00_01_11).unwrap());
        assert_eq!(SP, StandardRegister::from_trit4(0b00_00_01_00).unwrap());
        assert_eq!(FP, StandardRegister::from_trit4(0b00_00_01_01).unwrap());
        assert_eq!(RA, StandardRegister::from_trit4(0b00_01_11_11).unwrap());
        assert_eq!(A0, StandardRegister::from_trit4(0b00_01_11_00).unwrap());
        assert_eq!(A1, StandardRegister::from_trit4(0b00_01_11_01).unwrap());
        assert_eq!(A2, StandardRegister::from_trit4(0b00_01_00_11).unwrap());
        assert_eq!(A3, StandardRegister::from_trit4(0b00_01_00_00).unwrap());
        assert_eq!(A4, StandardRegister::from_trit4(0b00_01_00_01).unwrap());
        assert_eq!(A5, StandardRegister::from_trit4(0b00_01_01_11).unwrap());
        assert_eq!(S0, StandardRegister::from_trit4(0b00_01_01_00).unwrap());
        assert_eq!(S1, StandardRegister::from_trit4(0b00_01_01_01).unwrap());
        assert_eq!(S2, StandardRegister::from_trit4(0b01_11_11_11).unwrap());
        assert_eq!(S3, StandardRegister::from_trit4(0b01_11_11_00).unwrap());
        assert_eq!(S4, StandardRegister::from_trit4(0b01_11_11_01).unwrap());
        assert_eq!(S5, StandardRegister::from_trit4(0b01_11_00_11).unwrap());
        assert_eq!(T0, StandardRegister::from_trit4(0b01_11_00_00).unwrap());
        assert_eq!(T1, StandardRegister::from_trit4(0b01_11_00_01).unwrap());
        assert_eq!(T2, StandardRegister::from_trit4(0b01_11_01_11).unwrap());
        assert_eq!(T3, StandardRegister::from_trit4(0b01_11_01_00).unwrap());
        assert_eq!(T4, StandardRegister::from_trit4(0b01_11_01_01).unwrap());
        assert_eq!(T5, StandardRegister::from_trit4(0b01_00_11_11).unwrap());

        assert!(StandardRegister::from_trit4(0b00_00_00_11).is_err());
        assert!(StandardRegister::from_trit4(0b01_00_11_00).is_err());
    }

    #[test]
    fn system_register_from_trit4() {
        assert_eq!(EHA, SystemRegister::from_trit4(0b00_00_00_00).unwrap());
        assert_eq!(ERA, SystemRegister::from_trit4(0b00_00_00_01).unwrap());
        assert_eq!(EC, SystemRegister::from_trit4(0b00_00_01_11).unwrap());
        assert_eq!(ED, SystemRegister::from_trit4(0b00_00_01_00).unwrap());

        assert!(SystemRegister::from_trit4(0b00_00_00_11).is_err());
        assert!(SystemRegister::from_trit4(0b00_00_01_01).is_err());
    }
}
