use std::convert::{TryFrom, TryInto};
use std::cmp::Ordering;
use std::fmt;
use std::ops;
use std::ops::Mul;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use error::{Error, Result};
use trit;
use trit::Trit;
use hyte::{char_from_hyte, try_hyte_from_char};

pub const MIN_VALUE: i16 = -364;
pub const MAX_VALUE: i16 = 364;

const BITMASK: u16 = 0b11_11_11_11_11_11;
const HYTE_BITMASK: u8 = 0b11_11_11;
const SIGN_BITMASK: u16 = 0b10_10_10_10_10_10;
const HYTE_BIT_WIDTH: usize = 6;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq)]
pub struct Tryte(pub u16);

pub const ZERO: Tryte = Tryte(trit::BIN_ZERO);

impl Tryte {
    pub fn get_trit(self, i: usize) -> Trit {
        let shf = (i as u16) * 2;
        let bits = self.0 >> shf & trit::BITMASK;
        Trit(bits)
    }

    pub fn set_trit(self, i: usize, trit: Trit) -> Tryte {
        let shf = (i as u16) * 2;
        let bits = (self.0 | trit.0 << shf) & BITMASK;
        Tryte(bits)
    }

    pub fn from_bytes<R: ReadBytesExt>(reader: &mut R) -> Result<Tryte> {
        let bits = reader.read_u16::<LittleEndian>()?;
        let tryte = Tryte(bits);

        for i in 0..6 {
            let trit = tryte.get_trit(i);
            let trit_bits = trit.0;
            if trit_bits == trit::BIN_INVALID {
                return Err(Error::InvalidBitPattern(trit_bits as u64));
            }
        }

        Ok(tryte)
    }

    pub fn write_bytes<W: WriteBytesExt>(&self, writer: &mut W) -> Result<()> {
        Ok(writer.write_u16::<LittleEndian>(self.0)?)
    }

    fn from_hytes(low_hyte: u8, high_hyte: u8) -> Tryte {
        let bits = (high_hyte as u16) << HYTE_BIT_WIDTH | (low_hyte as u16);
        Tryte(bits)
    }

    fn hytes(self) -> (u8, u8) {
        (self.low(), self.high())
    }

    pub fn low(self) -> u8 {
        self.0 as u8 & HYTE_BITMASK
    }

    pub fn high(self) -> u8 {
        (self.0 >> HYTE_BIT_WIDTH) as u8 & HYTE_BITMASK
    }

    fn negation_bits(self) -> u16 {
        self.0 << 1 & SIGN_BITMASK
    }

    pub fn tcmp(self, rhs: Tryte) -> Tryte {
        zip(self, rhs, Trit::tcmp)
    }

    pub fn tmul(self, rhs: Tryte) -> Tryte {
        zip(self, rhs, Trit::mul)
    }

    pub fn add_with_carry(self, rhs: Tryte, carry: Trit) -> (Tryte, Trit) {
        let mut tryte = ZERO;
        let mut carry = carry;

        for i in 0..6 {
            let a = self.get_trit(i);
            let b = rhs.get_trit(i);
            let (c, _carry) = a.add_with_carry(b, carry);
            tryte = tryte.set_trit(i, c);
            carry = _carry;
        }

        (tryte, carry)
    }

    pub fn from_hyte_str(s: &str) -> Result<Tryte> {
        if s.len() != 2 {
            return Err(Error::InvalidDataLength(2, s.len()));
        }

        let mut chars = s.chars();
        let high_char = chars.next().ok_or_else(
            || Error::InvalidString(s.to_owned()),
        )?;
        let low_char = chars.next().ok_or_else(
            || Error::InvalidString(s.to_owned()),
        )?;
        let high_hyte = try_hyte_from_char(high_char)?;
        let low_hyte = try_hyte_from_char(low_char)?;
        let tryte = Tryte::from_hytes(low_hyte, high_hyte);
        Ok(tryte)
    }

    pub fn from_trit_str(s: &str) -> Result<Tryte> {
        if s.len() != 6 {
            return Err(Error::InvalidDataLength(6, s.len()));
        }

        let trits_result: Result<Vec<_>> = s.chars().rev().map(Trit::try_from).collect();
        let trits = trits_result?;
        Tryte::from_trits(&trits)
    }

    fn from_trits(trits: &[Trit]) -> Result<Tryte> {
        let mut tryte = ZERO;

        if trits.len() != 6 {
            return Err(Error::InvalidDataLength(6, trits.len()));
        }

        for (i, &trit) in trits.iter().enumerate() {
            tryte = tryte.set_trit(i, trit);
        }

        Ok(tryte)
    }

    fn fmt_hytes(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (low_hyte, high_hyte) = self.hytes();
        let low_char = char_from_hyte(low_hyte);
        let high_char = char_from_hyte(high_hyte);
        write!(f, "{}{}", high_char, low_char)
    }

    fn fmt_trits(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..6).rev() {
            let trit = self.get_trit(i);
            let c: char = trit.into();
            write!(f, "{}", c)?;
        }

        Ok(())
    }

    pub fn display_hytes(self) -> DisplayHytes {
        DisplayHytes(self)
    }

    pub fn display_trits(self) -> DisplayTrits {
        DisplayTrits(self)
    }
}

fn zip<F: Fn(Trit, Trit) -> Trit>(lhs: Tryte, rhs: Tryte, f: F) -> Tryte {
    let mut tryte = ZERO;

    for i in 0..6 {
        let a = lhs.get_trit(i);
        let b = rhs.get_trit(i);
        let c = f(a, b);
        tryte = tryte.set_trit(i, c);
    }

    tryte
}

impl From<Trit> for Tryte {
    fn from(trit: Trit) -> Tryte {
        Tryte(trit.0)
    }
}

impl TryInto<Trit> for Tryte {
    type Error = Error;

    fn try_into(self) -> Result<Trit> {
        let bits = self.0;
        if bits == trit::BIN_INVALID || bits > trit::BIN_NEG {
            Err(Error::InvalidBitPattern(bits as u64))
        } else {
            Ok(Trit(bits))
        }
    }
}

impl Into<i16> for Tryte {
    fn into(self) -> i16 {
        let mut n = 0i16;

        for i in (0..6).rev() {
            let trit = self.get_trit(i);
            let t: i16 = trit.into();
            n = n * 3 + t;
        }

        n
    }
}

impl TryFrom<i16> for Tryte {
    type Error = Error;

    fn try_from(n: i16) -> Result<Self> {
        if n < MIN_VALUE || MAX_VALUE < n {
            return Err(Error::IntegerOutOfBounds(
                MIN_VALUE as i64,
                MAX_VALUE as i64,
                n as i64,
            ));
        }

        let negative = n < 0;
        let mut n = n.abs();
        let mut tryte = ZERO;

        for i in 0..6 {
            let rem_trit = match n % 3 {
                1 => trit::POS,
                0 => trit::ZERO,
                _ => {
                    n += 1;
                    trit::NEG
                }
            };

            let trit = if negative { -rem_trit } else { rem_trit };
            tryte = tryte.set_trit(i, trit);
            n /= 3;
        }

        Ok(tryte)
    }
}

impl PartialOrd for Tryte {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut cmp_trit = trit::ZERO;

        for i in (0..6).rev() {
            let a = self.get_trit(i);
            let b = other.get_trit(i);
            cmp_trit = a.tcmp(b);

            if cmp_trit != trit::ZERO {
                break;
            }
        }

        Some(cmp_trit.into())
    }
}

impl ops::Neg for Tryte {
    type Output = Tryte;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ self.negation_bits();
        Tryte(bits)
    }
}

impl ops::Not for Tryte {
    type Output = Tryte;

    fn not(self) -> Self::Output {
        -self
    }
}

impl ops::BitAnd for Tryte {
    type Output = Tryte;

    fn bitand(self, rhs: Tryte) -> Self::Output {
        zip(self, rhs, Trit::bitand)
    }
}

impl ops::BitOr for Tryte {
    type Output = Tryte;

    fn bitor(self, rhs: Tryte) -> Self::Output {
        zip(self, rhs, Trit::bitor)
    }
}

impl ops::Add for Tryte {
    type Output = Tryte;

    fn add(self, rhs: Tryte) -> Self::Output {
        let (sum, _) = self.add_with_carry(rhs, trit::ZERO);
        sum
    }
}

pub struct DisplayHytes(Tryte);

impl fmt::Display for DisplayHytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_hytes(f)
    }
}

pub struct DisplayTrits(Tryte);

impl fmt::Display for DisplayTrits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt_trits(f)
    }
}
