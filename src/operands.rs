use constants::*;
use error::{Error, Result};
use trit::Trit;
use tryte;
use tryte::Tryte;
use registers::*;

const OPCODE_BITMASK: u16 = 0b00_00_00_00_11_11_11_11;

pub trait Operand: Sized {
    fn from_word(word: &[Tryte]) -> Result<Self>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Empty;

impl Operand for Empty {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        if word[0].0 & !OPCODE_BITMASK == 0 && word[1] == tryte::ZERO &&
            word[2] == tryte::ZERO && word[3] == tryte::ZERO
        {
            Ok(Empty)
        } else {
            Err(Error::InvalidInstruction("".to_owned()))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct R {
    src: StandardRegister,
}

impl Operand for R {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RR {
    lhs: StandardRegister,
    rhs: StandardRegister,
}

impl Operand for RR {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRR {
    dest: StandardRegister,
    lhs: StandardRegister,
    rhs: StandardRegister,
}

impl Operand for RRR {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RI {
    dest: StandardRegister,
    immediate: [Tryte; HALF_LEN],
}

impl Operand for RI {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRI {
    dest: StandardRegister,
    src: StandardRegister,
    immediate: [Tryte; HALF_LEN],
}

impl Operand for RRI {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Memory {
    dest: StandardRegister,
    src: StandardRegister,
    offset: [Tryte; HALF_LEN],
}

impl Operand for Memory {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Branch {
    src: StandardRegister,
    index: u8,
    hint: Trit,
    offset: [Tryte; HALF_LEN],
}

impl Operand for Branch {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Jump {
    offset: [Tryte; WORD_LEN],
}

impl Operand for Jump {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadSystem {
    dest: StandardRegister,
    src: SystemRegister,
}

impl Operand for LoadSystem {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreSystem {
    dest: StandardRegister,
    src: SystemRegister,
}

impl Operand for StoreSystem {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        unimplemented!()
    }
}
