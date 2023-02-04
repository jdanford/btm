use ternary::constants::{HALF_LEN, WORD_LEN};
use ternary::{tryte, Ternary, Trit, Tryte};

use crate::error::{Error, Result};
use crate::registers::{Register, StandardRegister, SystemRegister};

const TRIT4_BITMASK: u16 = 0b00_00_00_00_11_11_11_11;

pub trait Operand: Sized {
    fn from_word(word: &[Tryte]) -> Result<Self>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Empty;

impl Operand for Empty {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        if word[0].0 & !TRIT4_BITMASK == 0
            && word[1] == tryte::ZERO
            && word[2] == tryte::ZERO
            && word[3] == tryte::ZERO
        {
            Ok(Empty)
        } else {
            let mut bytes = Vec::new();
            word.write_trits(&mut bytes)?;
            let s = String::from_utf8_lossy(&bytes).into_owned();
            Err(ternary::Error::InvalidEncoding(s).into())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct R {
    pub src: StandardRegister,
}

impl Operand for R {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_src, _) = trit4_triple_from_half(half);
        let src = StandardRegister::from_trit4(trit4_src)?;
        Ok(Self { src })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RR {
    pub lhs: StandardRegister,
    pub rhs: StandardRegister,
}

impl Operand for RR {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_lhs, trit4_rhs) = trit4_triple_from_half(half);

        let lhs = StandardRegister::from_trit4(trit4_lhs)?;
        let rhs = StandardRegister::from_trit4(trit4_rhs)?;

        Ok(Self { lhs, rhs })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRR {
    pub dest: StandardRegister,
    pub lhs: StandardRegister,
    pub rhs: StandardRegister,
}

impl Operand for RRR {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, trit4_lhs) = trit4_triple_from_half(half);
        let trit4_rhs = word[HALF_LEN].low_trit4();

        let dest = StandardRegister::from_trit4(trit4_dest)?;
        let lhs = StandardRegister::from_trit4(trit4_lhs)?;
        let rhs = StandardRegister::from_trit4(trit4_rhs)?;

        Ok(Self { dest, lhs, rhs })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RI {
    pub dest: StandardRegister,
    pub immediate: [Tryte; HALF_LEN],
}

impl Operand for RI {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, _) = trit4_triple_from_half(half);

        let dest = StandardRegister::from_trit4(trit4_dest)?;
        let mut immediate = [tryte::ZERO; HALF_LEN];
        immediate.copy_from_slice(&word[HALF_LEN..]);

        Ok(Self { dest, immediate })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RRI {
    pub dest: StandardRegister,
    pub src: StandardRegister,
    pub immediate: [Tryte; HALF_LEN],
}

impl Operand for RRI {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, trit4_src) = trit4_triple_from_half(half);

        let dest = StandardRegister::from_trit4(trit4_dest)?;
        let src = StandardRegister::from_trit4(trit4_src)?;
        let mut immediate = [tryte::ZERO; HALF_LEN];
        immediate.copy_from_slice(&word[HALF_LEN..]);

        Ok(Self {
            dest,
            src,
            immediate,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Memory {
    pub dest: StandardRegister,
    pub src: StandardRegister,
    pub offset: [Tryte; HALF_LEN],
}

impl Operand for Memory {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, trit4_src) = trit4_triple_from_half(half);

        let dest = StandardRegister::from_trit4(trit4_dest)?;
        let src = StandardRegister::from_trit4(trit4_src)?;
        let mut offset = [tryte::ZERO; HALF_LEN];
        offset.copy_from_slice(&word[HALF_LEN..]);

        Ok(Self { dest, src, offset })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Branch {
    pub src: StandardRegister,
    pub index: u8,
    pub hint: Trit,
    pub offset: [Tryte; HALF_LEN],
}

impl Operand for Branch {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_src, trit4_index_hint) = trit4_triple_from_half(half);

        let src = StandardRegister::from_trit4(trit4_src)?;
        let index = (trit4_index_hint & tryte::HYTE_BITMASK) as u8;
        let hint = Trit::from_trit4(trit4_index_hint >> 6)?;
        let mut offset = [tryte::ZERO; HALF_LEN];
        offset.copy_from_slice(&word[HALF_LEN..]);

        Ok(Self {
            src,
            index,
            hint,
            offset,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Jump {
    pub offset: [Tryte; WORD_LEN],
}

impl Operand for Jump {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let offset = addr_from_word(word);
        Ok(Self { offset })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadSystem {
    pub dest: StandardRegister,
    pub src: SystemRegister,
}

impl Operand for LoadSystem {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, trit4_src) = trit4_triple_from_half(half);

        let dest = StandardRegister::from_trit4(trit4_dest)?;
        let src = SystemRegister::from_trit4(trit4_src)?;

        Ok(Self { dest, src })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreSystem {
    pub dest: SystemRegister,
    pub src: StandardRegister,
}

impl Operand for StoreSystem {
    fn from_word(word: &[Tryte]) -> Result<Self> {
        let half = &word[..HALF_LEN];
        let (_, trit4_dest, trit4_src) = trit4_triple_from_half(half);

        let dest = SystemRegister::from_trit4(trit4_dest)?;
        let src = StandardRegister::from_trit4(trit4_src)?;

        Ok(Self { dest, src })
    }
}

fn trit4_triple_from_half(half: &[Tryte]) -> (u8, u8, u8) {
    let trit6_a = half[0].0;
    let trit6_b = half[1].0;

    let trit4_a = trit6_a as u8;
    let trit4_b = ((trit6_a >> 8) | (trit6_b << 4)) as u8;
    let trit4_c = (trit6_b >> 4) as u8;
    (trit4_a, trit4_b, trit4_c)
}

fn addr_from_word(word: &[Tryte]) -> [Tryte; WORD_LEN] {
    let trits_0 = word[0].0;
    let trits_1 = word[1].0;
    let trits_2 = word[2].0;
    let trits_3 = word[3].0;

    let addr_trits_0 = (trits_0 >> 8 | trits_1 << 4) & tryte::BITMASK;
    let addr_trits_1 = (trits_1 >> 8 | trits_2 << 4) & tryte::BITMASK;
    let addr_trits_2 = (trits_2 >> 8 | trits_3 << 4) & tryte::BITMASK;
    let addr_trits_3 = (trits_3 >> 8) & tryte::BITMASK;

    [
        Tryte(addr_trits_0),
        Tryte(addr_trits_1),
        Tryte(addr_trits_2),
        Tryte(addr_trits_3),
    ]
}
