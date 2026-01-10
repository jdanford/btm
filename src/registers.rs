#![feature(generic_const_exprs)]

use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use ternary::{T24, Tryte, tables::TRIT4_TO_I8, tryte};

use crate::error::{Error, Result};

pub trait RegisterType {
    const COUNT: usize;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Standard;

impl RegisterType for Standard {
    const COUNT: usize = 24;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct System;

impl RegisterType for System {
    const COUNT: usize = 4;
}

pub type StandardRegister = Register<Standard>;
pub type SystemRegister = Register<System>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Register<R: RegisterType>(usize, PhantomData<R>);

impl<R: RegisterType> Register<R> {
    pub const fn from_trit4(trit4: u8) -> Result<Self> {
        let index_u8 = TRIT4_TO_I8[trit4 as usize] as u8;
        let index = index_u8 as usize;

        if index >= R::COUNT {
            return Err(Error::InvalidRegister(index_u8));
        }

        Ok(Self::from_index(index))
    }

    pub const fn from_index(i: usize) -> Self {
        Register(i, PhantomData)
    }

    pub const fn into_index(self) -> usize {
        self.0
    }
}

pub struct RegisterFile<R: RegisterType, const N: usize>([T24; N], PhantomData<R>);

impl<R: RegisterType, const N: usize> RegisterFile<R, N> {
    pub fn new() -> Self {
        Self([T24::ZERO; N], PhantomData)
    }
}

pub type StandardRegisters = RegisterFile<Standard, { Standard::COUNT }>;

impl Index<StandardRegister> for StandardRegisters {
    type Output = T24;

    fn index(&self, register: StandardRegister) -> &Self::Output {
        &self.0[register.into_index()]
    }
}

impl IndexMut<StandardRegister> for StandardRegisters {
    fn index_mut(&mut self, register: StandardRegister) -> &mut Self::Output {
        &mut self.0[register.into_index()]
    }
}

pub type SystemRegisters = RegisterFile<System, { System::COUNT }>;

impl Index<SystemRegister> for SystemRegisters {
    type Output = T24;

    fn index(&self, register: SystemRegister) -> &Self::Output {
        &self.0[register.into_index()]
    }
}

impl IndexMut<SystemRegister> for SystemRegisters {
    fn index_mut(&mut self, register: SystemRegister) -> &mut Self::Output {
        &mut self.0[register.into_index()]
    }
}

pub const ZERO: StandardRegister = StandardRegister::from_index(0);
pub const LO: StandardRegister = StandardRegister::from_index(1);
pub const HI: StandardRegister = StandardRegister::from_index(2);
pub const SP: StandardRegister = StandardRegister::from_index(3);
pub const FP: StandardRegister = StandardRegister::from_index(4);
pub const RA: StandardRegister = StandardRegister::from_index(5);
pub const A0: StandardRegister = StandardRegister::from_index(6);
pub const A1: StandardRegister = StandardRegister::from_index(7);
pub const A2: StandardRegister = StandardRegister::from_index(8);
pub const A3: StandardRegister = StandardRegister::from_index(9);
pub const A4: StandardRegister = StandardRegister::from_index(10);
pub const A5: StandardRegister = StandardRegister::from_index(11);
pub const S0: StandardRegister = StandardRegister::from_index(12);
pub const S1: StandardRegister = StandardRegister::from_index(13);
pub const S2: StandardRegister = StandardRegister::from_index(14);
pub const S3: StandardRegister = StandardRegister::from_index(15);
pub const S4: StandardRegister = StandardRegister::from_index(16);
pub const S5: StandardRegister = StandardRegister::from_index(17);
pub const T0: StandardRegister = StandardRegister::from_index(18);
pub const T1: StandardRegister = StandardRegister::from_index(19);
pub const T2: StandardRegister = StandardRegister::from_index(20);
pub const T3: StandardRegister = StandardRegister::from_index(21);
pub const T4: StandardRegister = StandardRegister::from_index(22);
pub const T5: StandardRegister = StandardRegister::from_index(23);
pub const EHA: SystemRegister = SystemRegister::from_index(0);
pub const ERA: SystemRegister = SystemRegister::from_index(1);
pub const EC: SystemRegister = SystemRegister::from_index(2);
pub const ED: SystemRegister = SystemRegister::from_index(3);

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
