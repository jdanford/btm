use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops;

use phf::phf_map;

use super::error::{Error, Result};
use super::tables::{
    TRIT2_TO_AND, TRIT2_TO_CMP, TRIT2_TO_OR, TRIT2_TO_PRODUCT, TRIT3_TO_SUM_AND_CARRY,
};

pub const BITMASK: u16 = 0b11;

pub const BIN_ZERO: u16 = 0b00;
pub const BIN_POS: u16 = 0b01;
pub const BIN_INVALID: u16 = 0b10;
pub const BIN_NEG: u16 = 0b11;

pub const CHAR_ZERO: char = '0';
pub const CHAR_POS: char = '1';
pub const CHAR_INVALID: char = '?';
pub const CHAR_NEG: char = 'T';

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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

    pub fn tcmp(self, rhs: Self) -> Self {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_CMP[i];
        Trit(bits)
    }

    pub fn add_with_carry(self, rhs: Self, carry_in: Self) -> (Self, Self) {
        let i = trit3_index(self, rhs, carry_in);
        let (sum, carry) = TRIT3_TO_SUM_AND_CARRY[i];
        (Trit(sum), Trit(carry))
    }

    pub fn into_index(self) -> usize {
        self.0 as usize
    }
}

fn trit2_index(a: Trit, b: Trit) -> usize {
    a.into_index() << 2 | b.into_index()
}

fn trit3_index(a: Trit, b: Trit, c: Trit) -> usize {
    a.into_index() << 4 | b.into_index() << 2 | c.into_index()
}

static TRIT_TO_I16: [i16; 4] = [0, 1, 0, -1];

impl Into<i16> for Trit {
    fn into(self) -> i16 {
        TRIT_TO_I16[self.into_index()]
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
        TRIT_TO_CHAR[self.into_index()]
    }
}

static CHAR_TO_TRIT: phf::Map<char, u16> = phf_map! {
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
        TRIT_TO_ORDERING[self.into_index()]
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

impl Ord for Trit {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_trit = self.tcmp(*other);
        cmp_trit.into()
    }
}

impl PartialOrd for Trit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Neg for Trit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ self.negation_bits();
        Trit(bits)
    }
}

impl ops::Not for Trit {
    type Output = Self;

    fn not(self) -> Self::Output {
        -self
    }
}

impl ops::BitAnd for Trit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_AND[i];
        Trit(bits)
    }
}

impl ops::BitOr for Trit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let i = trit2_index(self, rhs);
        let bits = TRIT2_TO_OR[i];
        Trit(bits)
    }
}

impl ops::Mul for Trit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trit_into_i16() {
        assert_eq!(-1_i16, Trit(BIN_NEG).into());
        assert_eq!(0_i16, Trit(BIN_ZERO).into());
        assert_eq!(1_i16, Trit(BIN_POS).into());
    }

    #[test]
    fn trit_from_i16() {
        assert_eq!(Ok(Trit(BIN_NEG)), Trit::try_from(-1));
        assert_eq!(Ok(Trit(BIN_ZERO)), Trit::try_from(0));
        assert_eq!(Ok(Trit(BIN_POS)), Trit::try_from(1));

        assert!(Trit::try_from(-2).is_err());
        assert!(Trit::try_from(2).is_err());
    }

    #[test]
    fn trit_into_char() {
        assert_eq!('T', Trit(BIN_NEG).into());
        assert_eq!('0', Trit(BIN_ZERO).into());
        assert_eq!('1', Trit(BIN_POS).into());
    }

    #[test]
    fn trit_from_char() {
        assert_eq!(Ok(Trit(BIN_NEG)), Trit::try_from('T'));
        assert_eq!(Ok(Trit(BIN_ZERO)), Trit::try_from('0'));
        assert_eq!(Ok(Trit(BIN_POS)), Trit::try_from('1'));

        assert!(Trit::try_from('t').is_err());
        assert!(Trit::try_from('S').is_err());
        assert!(Trit::try_from('2').is_err());
    }

    #[test]
    fn trit_negate() {
        assert_eq!(POS, -NEG);
        assert_eq!(ZERO, -ZERO);
        assert_eq!(NEG, -POS);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn trit_and() {
        assert_eq!(ZERO, ZERO & ZERO);
        assert_eq!(ZERO, ZERO & POS);
        assert_eq!(NEG, ZERO & NEG);
        assert_eq!(ZERO, POS & ZERO);
        assert_eq!(POS, POS & POS);
        assert_eq!(NEG, POS & NEG);
        assert_eq!(NEG, NEG & ZERO);
        assert_eq!(NEG, NEG & POS);
        assert_eq!(NEG, NEG & NEG);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn trit_or() {
        assert_eq!(ZERO, ZERO | ZERO);
        assert_eq!(POS, ZERO | POS);
        assert_eq!(ZERO, ZERO | NEG);
        assert_eq!(POS, POS | ZERO);
        assert_eq!(POS, POS | POS);
        assert_eq!(POS, POS | NEG);
        assert_eq!(ZERO, NEG | ZERO);
        assert_eq!(POS, NEG | POS);
        assert_eq!(NEG, NEG | NEG);
    }

    #[test]
    fn trit_add() {
        assert_eq!((ZERO, ZERO), ZERO.add_with_carry(ZERO, ZERO));
        assert_eq!((POS, ZERO), ZERO.add_with_carry(ZERO, POS));
        assert_eq!((NEG, ZERO), ZERO.add_with_carry(ZERO, NEG));
        assert_eq!((POS, ZERO), ZERO.add_with_carry(POS, ZERO));
        assert_eq!((NEG, POS), ZERO.add_with_carry(POS, POS));
        assert_eq!((ZERO, ZERO), ZERO.add_with_carry(POS, NEG));
        assert_eq!((NEG, ZERO), ZERO.add_with_carry(NEG, ZERO));
        assert_eq!((ZERO, ZERO), ZERO.add_with_carry(NEG, POS));
        assert_eq!((POS, NEG), ZERO.add_with_carry(NEG, NEG));
        assert_eq!((POS, ZERO), POS.add_with_carry(ZERO, ZERO));
        assert_eq!((NEG, POS), POS.add_with_carry(ZERO, POS));
        assert_eq!((ZERO, ZERO), POS.add_with_carry(ZERO, NEG));
        assert_eq!((NEG, POS), POS.add_with_carry(POS, ZERO));
        assert_eq!((ZERO, POS), POS.add_with_carry(POS, POS));
        assert_eq!((POS, ZERO), POS.add_with_carry(POS, NEG));
        assert_eq!((ZERO, ZERO), POS.add_with_carry(NEG, ZERO));
        assert_eq!((POS, ZERO), POS.add_with_carry(NEG, POS));
        assert_eq!((NEG, ZERO), POS.add_with_carry(NEG, NEG));
        assert_eq!((NEG, ZERO), NEG.add_with_carry(ZERO, ZERO));
        assert_eq!((ZERO, ZERO), NEG.add_with_carry(ZERO, POS));
        assert_eq!((POS, NEG), NEG.add_with_carry(ZERO, NEG));
        assert_eq!((ZERO, ZERO), NEG.add_with_carry(POS, ZERO));
        assert_eq!((POS, ZERO), NEG.add_with_carry(POS, POS));
        assert_eq!((NEG, ZERO), NEG.add_with_carry(POS, NEG));
        assert_eq!((POS, NEG), NEG.add_with_carry(NEG, ZERO));
        assert_eq!((NEG, ZERO), NEG.add_with_carry(NEG, POS));
        assert_eq!((ZERO, NEG), NEG.add_with_carry(NEG, NEG));
    }

    #[test]
    fn trit_mul() {
        assert_eq!(ZERO, ZERO * ZERO);
        assert_eq!(ZERO, ZERO * POS);
        assert_eq!(ZERO, ZERO * NEG);
        assert_eq!(ZERO, POS * ZERO);
        assert_eq!(POS, POS * POS);
        assert_eq!(NEG, POS * NEG);
        assert_eq!(ZERO, NEG * ZERO);
        assert_eq!(NEG, NEG * POS);
        assert_eq!(POS, NEG * NEG);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn trit_cmp() {
        assert!(ZERO == ZERO);
        assert!(ZERO < POS);
        assert!(ZERO > NEG);
        assert!(POS > ZERO);
        assert!(POS > NEG);
        assert!(POS == POS);
        assert!(NEG < ZERO);
        assert!(NEG < POS);
        assert!(NEG == NEG);
    }
}
