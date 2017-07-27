use std::convert::TryFrom;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

use phf;

use super::tables::*;
use super::error::{Error, Result};

pub const BITMASK: u16 = 0b11;

pub const BIN_ZERO: u16 = 0b00;
pub const BIN_POS: u16 = 0b01;
pub const BIN_INVALID: u16 = 0b10;
pub const BIN_NEG: u16 = 0b11;

pub const CHAR_ZERO: char = '0';
pub const CHAR_POS: char = '1';
pub const CHAR_INVALID: char = '?';
pub const CHAR_NEG: char = 'T';

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq)]
pub struct Trit(pub u16);

pub const ZERO: Trit = Trit(BIN_ZERO);
pub const POS: Trit = Trit(BIN_POS);
pub const NEG: Trit = Trit(BIN_NEG);

impl Trit {
    pub fn from_trit4(trit4: u8) -> Result<Self> {
        let trit_bits = trit4 as u16 & BITMASK;
        if trit_bits == BIN_INVALID {
            return Err(Error::InvalidBitPattern(trit_bits as u64));
        }

        Ok(Trit(trit_bits))
    }

    fn negation_bits(self) -> u16 {
        self.0 << 1 & BITMASK
    }

    pub fn tcmp(self, rhs: Trit) -> Trit {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_CMP[i];
        Trit(bits)
    }

    pub fn add_with_carry(self, rhs: Trit, carry: Trit) -> (Trit, Trit) {
        let i = trit3_index(self, rhs, carry);
        let (sum, carry) = TRIT3_TO_SUM_AND_CARRY[i];
        (Trit(sum), Trit(carry))
    }
}

fn trit2_index(a: Trit, b: Trit) -> usize {
    (a.0 << 2 | b.0) as usize
}

fn trit3_index(a: Trit, b: Trit, c: Trit) -> usize {
    (a.0 << 4 | b.0 << 2 | c.0) as usize
}

static TRIT_TO_I16: [i16; 4] = [0, 1, 0, -1];

impl Into<i16> for Trit {
    fn into(self) -> i16 {
        TRIT_TO_I16[self.0 as usize]
    }
}

static U16_TO_TRIT: [u16; 3] = [BIN_NEG, BIN_ZERO, BIN_POS];

impl TryFrom<i16> for Trit {
    type Error = Error;

    fn try_from(n: i16) -> Result<Self> {
        let uint = (n + 1) as usize;
        if uint < 3 {
            let bits = U16_TO_TRIT[uint];
            Ok(Trit(bits))
        } else {
            Err(Error::IntegerOutOfBounds(-1, 1, n as i64))
        }
    }
}

static TRIT_TO_CHAR: [char; 4] = [CHAR_ZERO, CHAR_POS, CHAR_INVALID, CHAR_NEG];

impl Into<char> for Trit {
    fn into(self) -> char {
        TRIT_TO_CHAR[self.0 as usize]
    }
}

static CHAR_TO_TRIT: phf::Map<char, u16> =
    phf_map! {
    'T' => BIN_NEG,
    '0' => BIN_ZERO,
    '1' => BIN_POS,
};

impl TryFrom<char> for Trit {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        if let Some(&bits) = CHAR_TO_TRIT.get(&c) {
            Ok(Trit(bits))
        } else {
            Err(Error::InvalidCharacter(c))
        }
    }
}

static TRIT_TO_ORDERING: [Ordering; 4] = [
    Ordering::Equal,
    Ordering::Greater,
    Ordering::Equal,
    Ordering::Less,
];

impl Into<Ordering> for Trit {
    fn into(self) -> Ordering {
        TRIT_TO_ORDERING[self.0 as usize]
    }
}

impl TryFrom<Ordering> for Trit {
    type Error = Error;

    fn try_from(ordering: Ordering) -> Result<Self> {
        match ordering {
            Ordering::Less => Ok(NEG),
            Ordering::Equal => Ok(ZERO),
            Ordering::Greater => Ok(POS),
        }
    }
}

impl PartialOrd for Trit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_trit = self.tcmp(*other);
        Some(cmp_trit.into())
    }
}

impl ops::Neg for Trit {
    type Output = Trit;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ self.negation_bits();
        Trit(bits)
    }
}

impl ops::Not for Trit {
    type Output = Trit;

    fn not(self) -> Self::Output {
        -self
    }
}

impl ops::BitAnd for Trit {
    type Output = Trit;

    fn bitand(self, rhs: Trit) -> Self::Output {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_AND[i];
        Trit(bits)
    }
}

impl ops::BitOr for Trit {
    type Output = Trit;

    fn bitor(self, rhs: Trit) -> Self::Output {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_OR[i];
        Trit(bits)
    }
}

impl ops::Mul for Trit {
    type Output = Trit;

    fn mul(self, rhs: Trit) -> Self::Output {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_PRODUCT[i];
        Trit(bits)
    }
}

impl fmt::Debug for Trit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trit({:02b})", self.0)
    }
}
