use std::convert::TryInto;
use std::fmt;
use std::io;
use std::ops;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::constants::*;
use super::error::{Error, Result};
use super::trit;
use super::trit::Trit;
use super::hyte::{char_from_hyte, try_hyte_from_char};

pub use super::constants::TRYTE_TRIT_LEN as TRIT_LEN;

pub const BITMASK: u16 = 0b11_11_11_11_11_11;
pub const HYTE_BITMASK: u8 = 0b11_11_11;
const SIGN_BITMASK: u16 = 0b10_10_10_10_10_10;

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
        let bits = (high_hyte as u16) << HYTE_BIT_LEN | (low_hyte as u16);
        Tryte(bits)
    }

    fn low_hyte(self) -> u8 {
        self.0 as u8 & HYTE_BITMASK
    }

    fn high_hyte(self) -> u8 {
        (self.0 >> HYTE_BIT_LEN) as u8 & HYTE_BITMASK
    }

    fn hytes(self) -> (u8, u8) {
        (self.low_hyte(), self.high_hyte())
    }

    pub fn low_trit4(self) -> u8 {
        self.0 as u8
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

    pub fn write_hytes<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
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
        let (low_hyte, high_hyte) = self.hytes();
        let low_char = char_from_hyte(low_hyte);
        let high_char = char_from_hyte(high_hyte);
        write!(f, "{}{}", high_char, low_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_constants::*;

    use std::io::Cursor;

    #[test]
    fn tryte_into_trit() {
        assert_eq!(Ok(trit::NEG), TRYTE_NEG1.try_into());
        assert_eq!(Ok(trit::ZERO), TRYTE_0.try_into());
        assert_eq!(Ok(trit::POS), TRYTE_1.try_into());

        assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_NEG64).is_err());
        assert!(<Tryte as TryInto<Trit>>::try_into(TRYTE_64).is_err());
    }

    #[test]
    fn tryte_from_trit() {
        assert_eq!(TRYTE_NEG1, trit::NEG.into());
        assert_eq!(TRYTE_0, trit::ZERO.into());
        assert_eq!(TRYTE_1, trit::POS.into());
    }

    #[test]
    fn tryte_from_bytes() {
        assert_eq!(Ok(TRYTE_MIN), from_bytes(0b11_11_11_11, 0b00_00_11_11));
        assert_eq!(Ok(TRYTE_NEG64), from_bytes(0b01_11_00_11, 0b00_00_00_11));
        assert_eq!(Ok(TRYTE_NEG1), from_bytes(0b00_00_00_11, 0b00_00_00_00));
        assert_eq!(Ok(TRYTE_0), from_bytes(0b00_00_00_00, 0b00_00_00_00));
        assert_eq!(Ok(TRYTE_1), from_bytes(0b00_00_00_01, 0b00_00_00_00));
        assert_eq!(Ok(TRYTE_64), from_bytes(0b11_01_00_01, 0b00_00_00_01));
        assert_eq!(Ok(TRYTE_MAX), from_bytes(0b01_01_01_01, 0b00_00_01_01));

        assert!(from_bytes(0b01_01_10_01, 0b00_00_01_01).is_err());
    }

    fn from_bytes(low: u8, high: u8) -> Result<Tryte> {
        let mut cursor = Cursor::new(vec![low, high]);
        Ok(Tryte::from_bytes(&mut cursor)?)
    }

    #[test]
    fn tryte_write_bytes() {
        assert_eq!(vec![0b11_11_11_11, 0b00_00_11_11], get_bytes(TRYTE_MIN));
        assert_eq!(vec![0b01_11_00_11, 0b00_00_00_11], get_bytes(TRYTE_NEG64));
        assert_eq!(vec![0b00_00_00_11, 0b00_00_00_00], get_bytes(TRYTE_NEG1));
        assert_eq!(vec![0b00_00_00_00, 0b00_00_00_00], get_bytes(TRYTE_0));
        assert_eq!(vec![0b00_00_00_01, 0b00_00_00_00], get_bytes(TRYTE_1));
        assert_eq!(vec![0b11_01_00_01, 0b00_00_00_01], get_bytes(TRYTE_64));
        assert_eq!(vec![0b01_01_01_01, 0b00_00_01_01], get_bytes(TRYTE_MAX));
    }

    fn get_bytes(tryte: Tryte) -> Vec<u8> {
        let mut bytes = vec![];
        tryte.write_bytes(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn tryte_display_hytes() {
        assert_eq!("mm", format!("{}", TRYTE_MIN));
        assert_eq!("bj", format!("{}", TRYTE_NEG64));
        assert_eq!("0a", format!("{}", TRYTE_NEG1));
        assert_eq!("00", format!("{}", TRYTE_0));
        assert_eq!("0A", format!("{}", TRYTE_1));
        assert_eq!("BJ", format!("{}", TRYTE_64));
        assert_eq!("MM", format!("{}", TRYTE_MAX));
    }

    #[test]
    fn tryte_from_hyte_str() {
        assert_eq!(Ok(TRYTE_MIN), Tryte::from_hyte_str("mm"));
        assert_eq!(Ok(TRYTE_NEG64), Tryte::from_hyte_str("bj"));
        assert_eq!(Ok(TRYTE_NEG1), Tryte::from_hyte_str("0a"));
        assert_eq!(Ok(TRYTE_0), Tryte::from_hyte_str("00"));
        assert_eq!(Ok(TRYTE_1), Tryte::from_hyte_str("0A"));
        assert_eq!(Ok(TRYTE_64), Tryte::from_hyte_str("BJ"));
        assert_eq!(Ok(TRYTE_MAX), Tryte::from_hyte_str("MM"));

        assert!(Tryte::from_hyte_str("").is_err());
        assert!(Tryte::from_hyte_str("M").is_err());
        assert!(Tryte::from_hyte_str("MMM").is_err());
        assert!(Tryte::from_hyte_str("NN").is_err());
    }

    #[test]
    fn tryte_negate() {
        assert_eq!(TRYTE_MIN, -TRYTE_MAX);
        assert_eq!(TRYTE_NEG64, -TRYTE_64);
        assert_eq!(TRYTE_NEG1, -TRYTE_1);
        assert_eq!(TRYTE_0, -TRYTE_0);
        assert_eq!(TRYTE_1, -TRYTE_NEG1);
        assert_eq!(TRYTE_64, -TRYTE_NEG64);
        assert_eq!(TRYTE_MAX, -TRYTE_MIN);

        assert_eq!(TRYTE_MAX, -TRYTE_MIN);
        assert_eq!(TRYTE_64, -TRYTE_NEG64);
        assert_eq!(TRYTE_1, -TRYTE_NEG1);
        assert_eq!(TRYTE_0, -TRYTE_0);
        assert_eq!(TRYTE_NEG1, -TRYTE_1);
        assert_eq!(TRYTE_NEG64, -TRYTE_64);
        assert_eq!(TRYTE_MIN, -TRYTE_MAX);
    }

    #[test]
    fn tryte_add() {
        assert_eq!(
            (TRYTE_0, trit::ZERO),
            TRYTE_1.add_with_carry(TRYTE_NEG1, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_0, trit::ZERO),
            TRYTE_64.add_with_carry(TRYTE_NEG64, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_0, trit::ZERO),
            TRYTE_MAX.add_with_carry(TRYTE_MIN, trit::ZERO)
        );

        assert_eq!(
            (TRYTE_MIN, trit::ZERO),
            TRYTE_MIN.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_NEG64, trit::ZERO),
            TRYTE_NEG64.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_NEG1, trit::ZERO),
            TRYTE_NEG1.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_0, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_1, trit::ZERO),
            TRYTE_1.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_64, trit::ZERO),
            TRYTE_64.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_MAX, trit::ZERO),
            TRYTE_MAX.add_with_carry(TRYTE_0, trit::ZERO)
        );

        assert_eq!(
            (TRYTE_MIN, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_MIN, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_NEG64, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_NEG64, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_NEG1, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_NEG1, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_0, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_0, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_1, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_1, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_64, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_64, trit::ZERO)
        );
        assert_eq!(
            (TRYTE_MAX, trit::ZERO),
            TRYTE_0.add_with_carry(TRYTE_MAX, trit::ZERO)
        );
    }
}
