use std::convert::TryInto;
use std::fmt;
use std::ops;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use error::{Error, Result};
use trit;
use trit::Trit;
use hyte::{char_from_hyte, try_hyte_from_char};

pub const TRIT_LEN: usize = 6;

const BITMASK: u16 = 0b11_11_11_11_11_11;
const HYTE_BITMASK: u8 = 0b11_11_11;
const SIGN_BITMASK: u16 = 0b10_10_10_10_10_10;
const HYTE_BIT_WIDTH: usize = 6;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
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
        let zero_bits = !(0b11 << shf);
        let tryte_bits = self.0 & zero_bits;
        let trit_bits = trit.0 << shf;
        let bits = (tryte_bits | trit_bits) & BITMASK;
        Tryte(bits)
    }

    pub fn from_bytes<R: ReadBytesExt>(reader: &mut R) -> Result<Tryte> {
        let bits = reader.read_u16::<LittleEndian>()?;
        let tryte = Tryte(bits);

        for i in 0..TRIT_LEN {
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

    fn low(self) -> u8 {
        self.0 as u8 & HYTE_BITMASK
    }

    fn high(self) -> u8 {
        (self.0 >> HYTE_BIT_WIDTH) as u8 & HYTE_BITMASK
    }

    fn negation_bits(self) -> u16 {
        self.0 << 1 & SIGN_BITMASK
    }

    pub fn add_with_carry(self, rhs: Tryte, carry: Trit) -> (Tryte, Trit) {
        let mut tryte = ZERO;
        let mut carry = carry;

        for i in 0..TRIT_LEN {
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

    pub fn write_hytes<W: fmt::Write>(&self, mut writer: W) -> fmt::Result {
        let (low_hyte, high_hyte) = self.hytes();
        let low_char = char_from_hyte(low_hyte);
        let high_char = char_from_hyte(high_hyte);
        write!(writer, "{}{}", high_char, low_char)
    }
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

impl ops::Neg for Tryte {
    type Output = Tryte;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ self.negation_bits();
        Tryte(bits)
    }
}

impl fmt::Debug for Tryte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tryte({:012b})", self.0)
    }
}

impl fmt::Display for Tryte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_hytes(f)
    }
}
