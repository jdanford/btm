use std::ops::{Index, IndexMut};

use constants::*;
use tables::TRIT4_TO_U8;
use error::{Error, Result};
use tryte;
use tryte::Tryte;

pub trait Register: Sized {
    const COUNT: usize;

    fn from_trit4(trit4: u8) -> Result<Self> {
        let i = TRIT4_TO_U8[trit4 as usize];
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
