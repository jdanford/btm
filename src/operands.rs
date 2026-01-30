#![allow(clippy::upper_case_acronyms)]

use std::convert::{TryFrom, TryInto};

use ternary::{T12, T24, Trit, Tryte, tryte};

use crate::error::{Error, Result};
use crate::registers::Register;

const TRIT4_BITMASK: u16 = 0b00_00_00_00_11_11_11_11;

pub trait Operand: Sized {
    fn from_word(word: T24) -> Result<Self>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Empty;

impl Operand for Empty {
    fn from_word(word: T24) -> Result<Self> {
        let trytes = word.into_trytes();
        if trytes[0].into_raw() & !TRIT4_BITMASK == 0
            && trytes[1] == Tryte::ZERO
            && trytes[2] == Tryte::ZERO
            && trytes[3] == Tryte::ZERO
        {
            Ok(Empty)
        } else {
            Err(ternary::Error::InvalidEncoding(trytes.into()).into())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct R {
    pub src: Register,
}

impl Operand for R {
    fn from_word(word: T24) -> Result<Self> {
        let half = word.resize();
        let (_, trit4_src, _) = half.trit4_triple();
        let src = Register::from_trit4(trit4_src)?;
        Ok(Self { src })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RR {
    pub lhs: Register,
    pub rhs: Register,
}

impl Operand for RR {
    fn from_word(word: T24) -> Result<Self> {
        let half = word.resize();
        let (_, trit4_lhs, trit4_rhs) = half.trit4_triple();

        let lhs = Register::from_trit4(trit4_lhs)?;
        let rhs = Register::from_trit4(trit4_rhs)?;

        Ok(Self { lhs, rhs })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRR {
    pub dest: Register,
    pub lhs: Register,
    pub rhs: Register,
}

impl Operand for RRR {
    fn from_word(word: T24) -> Result<Self> {
        let half = word.resize();
        let (_, trit4_dest, trit4_lhs) = half.trit4_triple();
        let trit4_rhs = word.into_trytes()[2].low_trit4();

        let dest = Register::from_trit4(trit4_dest)?;
        let lhs = Register::from_trit4(trit4_lhs)?;
        let rhs = Register::from_trit4(trit4_rhs)?;

        Ok(Self { dest, lhs, rhs })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RI {
    pub dest: Register,
    pub immediate: T12,
}

impl Operand for RI {
    fn from_word(word: T24) -> Result<Self> {
        let (lo, immediate) = word.t12_pair();
        let (_, trit4_dest, _) = lo.trit4_triple();
        let dest = Register::from_trit4(trit4_dest)?;
        Ok(Self { dest, immediate })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRI {
    pub dest: Register,
    pub src: Register,
    pub immediate: T12,
}

impl Operand for RRI {
    fn from_word(word: T24) -> Result<Self> {
        let (lo, immediate) = word.t12_pair();
        let (_, trit4_dest, trit4_src) = lo.trit4_triple();

        let dest = Register::from_trit4(trit4_dest)?;
        let src = Register::from_trit4(trit4_src)?;

        Ok(Self {
            dest,
            src,
            immediate,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRO {
    pub dest: Register,
    pub src: Register,
    pub offset: T12,
}

impl Operand for RRO {
    fn from_word(word: T24) -> Result<Self> {
        let (lo, offset) = word.t12_pair();
        let (_, trit4_dest, trit4_src) = lo.trit4_triple();

        let dest = Register::from_trit4(trit4_dest)?;
        let src = Register::from_trit4(trit4_src)?;

        Ok(Self { dest, src, offset })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RO {
    pub src: Register,
    pub offset: T24,
}

impl Operand for RO {
    fn from_word(word: T24) -> Result<Self> {
        let (lo, _) = word.t12_pair();
        let offset = word >> 8;

        let (_, trit4_src, _) = lo.trit4_triple();
        let src = Register::from_trit4(trit4_src)?;

        Ok(Self { src, offset })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct O {
    pub offset: T24,
}

impl Operand for O {
    fn from_word(word: T24) -> Result<Self> {
        let offset = word >> 8;
        Ok(Self { offset })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A {
    pub addr: T24,
}

impl Operand for A {
    fn from_word(word: T24) -> Result<Self> {
        let addr = word >> 4;
        Ok(Self { addr })
    }
}
