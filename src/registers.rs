use std::ops::{Index, IndexMut};

use error::{Error, Result};
use ternary::constants::*;
use ternary::tables::TRIT4_TO_I8;
use ternary::{tryte, Tryte};

pub trait Register: Sized {
    const COUNT: usize;

    fn from_trit4(trit4: u8) -> Result<Self> {
        let i = TRIT4_TO_I8[trit4 as usize] as u8;
        if i as usize >= Self::COUNT {
            return Err(Error::InvalidRegister(i));
        }

        Ok(Self::from_index(i as usize))
    }

    fn into_indices(&self) -> (usize, usize) {
        let i = self.into_index();
        let j = i + WORD_LEN;
        (i, j)
    }

    fn from_index(usize) -> Self;
    fn into_index(&self) -> usize;
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

    fn into_index(&self) -> usize {
        self.0 as usize
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

    fn into_index(&self) -> usize {
        self.0 as usize + StandardRegister::COUNT
    }
}

const TOTAL_COUNT: usize = StandardRegister::COUNT + SystemRegister::COUNT;
const TOTAL_LEN: usize = TOTAL_COUNT * WORD_LEN;

pub struct RegisterFile {
    registers: [Tryte; TOTAL_LEN],
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile { registers: [tryte::ZERO; TOTAL_LEN] }
    }
}

impl<R: Register> Index<R> for RegisterFile {
    type Output = [Tryte];

    fn index(&self, register: R) -> &Self::Output {
        let (i, j) = register.into_indices();
        &self.registers[i..j]
    }
}

impl<R: Register> IndexMut<R> for RegisterFile {
    fn index_mut(&mut self, register: R) -> &mut Self::Output {
        let (i, j) = register.into_indices();
        &mut self.registers[i..j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_register_from_trit4() {
        assert_eq!(Ok(ZERO), StandardRegister::from_trit4(0b00_00_00_00));
        assert_eq!(Ok(LO), StandardRegister::from_trit4(0b00_00_00_01));
        assert_eq!(Ok(HI), StandardRegister::from_trit4(0b00_00_01_11));
        assert_eq!(Ok(SP), StandardRegister::from_trit4(0b00_00_01_00));
        assert_eq!(Ok(FP), StandardRegister::from_trit4(0b00_00_01_01));
        assert_eq!(Ok(RA), StandardRegister::from_trit4(0b00_01_11_11));
        assert_eq!(Ok(A0), StandardRegister::from_trit4(0b00_01_11_00));
        assert_eq!(Ok(A1), StandardRegister::from_trit4(0b00_01_11_01));
        assert_eq!(Ok(A2), StandardRegister::from_trit4(0b00_01_00_11));
        assert_eq!(Ok(A3), StandardRegister::from_trit4(0b00_01_00_00));
        assert_eq!(Ok(A4), StandardRegister::from_trit4(0b00_01_00_01));
        assert_eq!(Ok(A5), StandardRegister::from_trit4(0b00_01_01_11));
        assert_eq!(Ok(S0), StandardRegister::from_trit4(0b00_01_01_00));
        assert_eq!(Ok(S1), StandardRegister::from_trit4(0b00_01_01_01));
        assert_eq!(Ok(S2), StandardRegister::from_trit4(0b01_11_11_11));
        assert_eq!(Ok(S3), StandardRegister::from_trit4(0b01_11_11_00));
        assert_eq!(Ok(S4), StandardRegister::from_trit4(0b01_11_11_01));
        assert_eq!(Ok(S5), StandardRegister::from_trit4(0b01_11_00_11));
        assert_eq!(Ok(T0), StandardRegister::from_trit4(0b01_11_00_00));
        assert_eq!(Ok(T1), StandardRegister::from_trit4(0b01_11_00_01));
        assert_eq!(Ok(T2), StandardRegister::from_trit4(0b01_11_01_11));
        assert_eq!(Ok(T3), StandardRegister::from_trit4(0b01_11_01_00));
        assert_eq!(Ok(T4), StandardRegister::from_trit4(0b01_11_01_01));
        assert_eq!(Ok(T5), StandardRegister::from_trit4(0b01_00_11_11));

        assert!(StandardRegister::from_trit4(0b00_00_00_11).is_err());
        assert!(StandardRegister::from_trit4(0b01_00_11_00).is_err());
    }

    #[test]
    fn system_register_from_trit4() {
        assert_eq!(Ok(EHA), SystemRegister::from_trit4(0b00_00_00_00));
        assert_eq!(Ok(ERA), SystemRegister::from_trit4(0b00_00_00_01));
        assert_eq!(Ok(EC), SystemRegister::from_trit4(0b00_00_01_11));
        assert_eq!(Ok(ED), SystemRegister::from_trit4(0b00_00_01_00));

        assert!(SystemRegister::from_trit4(0b00_00_00_11).is_err());
        assert!(SystemRegister::from_trit4(0b00_00_01_01).is_err());
    }
}
