pub mod error;
pub mod constants;
pub mod test_constants;
pub mod tables;
pub mod trit;
mod hyte;
pub mod tryte;
pub mod text;

use std::convert::TryFrom;
use std::io;
use std::ops::{BitAnd, BitOr, Mul, Neg};

use byteorder::{ReadBytesExt, WriteBytesExt};

pub use self::error::{Error, Result};
pub use self::trit::Trit;
pub use self::tryte::Tryte;

pub trait Ternary {
    fn trit_len(&self) -> usize;
    fn tryte_len(&self) -> usize;
    fn get_trit(&self, usize) -> Trit;
    fn set_trit(&mut self, usize, trit: Trit);
    fn get_tryte(&self, usize) -> Tryte;
    fn set_tryte(&mut self, usize, tryte: Tryte);

    fn range(&self) -> i64 {
        let base = 3i64;
        let exp = self.trit_len() as u32;
        base.pow(exp)
    }

    fn min_value(&self) -> i64 {
        -(self.range() - 1) / 2
    }

    fn max_value(&self) -> i64 {
        (self.range() - 1) / 2
    }

    fn clear(&mut self) {
        for i in 0..self.tryte_len() {
            self.set_tryte(i, tryte::ZERO);
        }
    }

    fn read_bytes<R: ReadBytesExt>(&mut self, reader: &mut R) -> Result<()> {
        for i in 0..self.tryte_len() {
            let tryte = Tryte::from_bytes(reader)?;
            self.set_tryte(i, tryte);
        }

        Ok(())
    }

    fn write_bytes<W: WriteBytesExt>(&self, mut writer: &mut W) -> Result<()> {
        for i in 0..self.tryte_len() {
            let tryte = self.get_tryte(i);
            tryte.write_bytes(writer)?;
        }

        Ok(())
    }

    fn into_i64(&self) -> i64 {
        let mut n = 0i64;

        for i in (0..self.trit_len()).rev() {
            let trit = self.get_trit(i);
            let t: i16 = trit.into();
            n = n * 3 + t as i64;
        }

        n
    }

    fn read_i64(&mut self, n: i64) -> Result<()> {
        if n < self.min_value() || self.max_value() < n {
            return Err(Error::IntegerOutOfBounds(
                self.min_value(),
                self.max_value(),
                n,
            ));
        }

        let sign_trit = if n < 0 { trit::NEG } else { trit::POS };
        let mut n = n.abs();

        for i in 0..self.trit_len() {
            let rem_trit = match n % 3 {
                1 => trit::POS,
                0 => trit::ZERO,
                _ => {
                    n += 1;
                    trit::NEG
                }
            };

            let trit = sign_trit * rem_trit;
            self.set_trit(i, trit);
            n /= 3;
        }

        Ok(())
    }

    fn read_hytes(&mut self, s: &str) -> Result<()> {
        let len = self.tryte_len() * 2;
        if s.len() != len {
            return Err(Error::InvalidDataLength(len, s.len()));
        }

        let mut s = s;
        for i in (0..self.tryte_len()).rev() {
            let (substr, _s) = s.split_at(2);
            s = _s;
            let tryte = Tryte::from_hyte_str(substr)?;
            self.set_tryte(i, tryte);
        }

        Ok(())
    }

    fn write_hytes<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        for i in (0..self.tryte_len()).rev() {
            let tryte = self.get_tryte(i);
            tryte.write_hytes(writer)?;
        }

        Ok(())
    }

    fn read_trits(&mut self, s: &str) -> Result<()> {
        if s.len() != self.trit_len() {
            return Err(Error::InvalidDataLength(self.trit_len(), s.len()));
        }

        for (i, c) in s.chars().rev().enumerate() {
            let trit = Trit::try_from(c)?;
            self.set_trit(i, trit);
        }

        Ok(())
    }

    fn write_trits<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        for i in (0..self.trit_len()).rev() {
            let trit = self.get_trit(i);
            let c: char = trit.into();
            write!(writer, "{}", c)?;
        }

        Ok(())
    }
}

pub fn compare<T: Ternary + ?Sized>(lhs: &T, rhs: &T) -> Trit {
    let mut cmp_trit = trit::ZERO;

    for i in (0..lhs.trit_len()).rev() {
        let a = lhs.get_trit(i);
        let b = rhs.get_trit(i);
        cmp_trit = a.tcmp(b);

        if cmp_trit != trit::ZERO {
            break;
        }
    }

    cmp_trit
}

pub fn negate<T: Ternary + ?Sized>(dest: &mut T, src: &T) {
    zip_trytes(dest, src, Tryte::neg)
}

pub fn and<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T) {
    zip2_trits(dest, lhs, rhs, Trit::bitand)
}

pub fn or<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T) {
    zip2_trits(dest, lhs, rhs, Trit::bitor)
}

pub fn tcmp<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T) {
    zip2_trits(dest, lhs, rhs, Trit::tcmp)
}

pub fn tmul<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T) {
    zip2_trits(dest, lhs, rhs, Trit::mul)
}

fn read_trits<T: Ternary + ?Sized>(dest: &mut T, trits: &[Trit]) -> Result<()> {
    if trits.len() != dest.trit_len() {
        return Err(Error::InvalidDataLength(dest.trit_len(), trits.len()));
    }

    for (i, &trit) in trits.iter().enumerate() {
        dest.set_trit(i, trit);
    }

    Ok(())
}

pub fn add<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T, carry: Trit) -> Trit {
    let mut carry = carry;

    for i in 0..lhs.trit_len() {
        let a = lhs.get_trit(i);
        let b = rhs.get_trit(i);
        let (c, _carry) = a.add_with_carry(b, carry);
        carry = _carry;
        dest.set_trit(i, c);
    }

    carry
}

pub fn multiply<T: Ternary + ?Sized>(dest: &mut T, lhs: &T, rhs: &T) {
    let len = rhs.trit_len();
    for i in 0..len {
        let sign = rhs.get_trit(i);
        let carry = add_mul(dest, lhs, sign, i);
        dest.set_trit(i + len, carry);
    }
}

fn add_mul<T: Ternary + ?Sized>(dest: &mut T, src: &T, sign: Trit, offset: usize) -> Trit {
    let mut carry = trit::ZERO;

    for i in 0..src.trit_len() {
        let a = dest.get_trit(i + offset);
        let b = src.get_trit(i);
        let (c, _carry) = a.add_with_carry(b * sign, carry);
        carry = _carry;
        dest.set_trit(i + offset, c);
    }

    carry
}

pub fn shift<T: Ternary + ?Sized>(dest: &mut T, src: &T, offset: isize) {
    let src_len = src.trit_len();
    let dest_len = src_len * 3;
    let dest_offset = offset + src_len as isize;

    for i in 0..src_len {
        let i_dest = i as isize + dest_offset;
        if i_dest < 0 || dest_len as isize <= i_dest {
            continue;
        }

        let trit = src.get_trit(i);
        dest.set_trit(i_dest as usize, trit);
    }
}

fn zip_trits<T, F>(dest: &mut T, lhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Trit) -> Trit,
{
    for i in 0..lhs.trit_len() {
        let trit = lhs.get_trit(i);
        dest.set_trit(i, f(trit));
    }
}

fn zip_trytes<T, F>(dest: &mut T, lhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Tryte) -> Tryte,
{
    for i in 0..lhs.tryte_len() {
        let tryte = lhs.get_tryte(i);
        dest.set_tryte(i, f(tryte));
    }
}

fn zip2_trits<T, F>(dest: &mut T, lhs: &T, rhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Trit, Trit) -> Trit,
{
    for i in 0..rhs.trit_len() {
        let a = lhs.get_trit(i);
        let b = rhs.get_trit(i);
        let c = f(a, b);
        dest.set_trit(i, c);
    }
}

fn zip2_trytes<T, F>(dest: &mut T, lhs: &T, rhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Tryte, Tryte) -> Tryte,
{
    for i in 0..rhs.tryte_len() {
        let a = lhs.get_tryte(i);
        let b = rhs.get_tryte(i);
        let c = f(a, b);
        dest.set_tryte(i, c);
    }
}

fn mutate_trits<T, F>(lhs: &mut T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Trit) -> Trit,
{
    for i in 0..lhs.trit_len() {
        let trit = lhs.get_trit(i);
        lhs.set_trit(i, f(trit));
    }
}

fn mutate_trytes<T, F>(lhs: &mut T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Tryte) -> Tryte,
{
    for i in 0..lhs.tryte_len() {
        let tryte = lhs.get_tryte(i);
        lhs.set_tryte(i, f(tryte));
    }
}

fn mutate2_trits<T, F>(lhs: &mut T, rhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Trit, Trit) -> Trit,
{
    for i in 0..rhs.trit_len() {
        let a = lhs.get_trit(i);
        let b = rhs.get_trit(i);
        let c = f(a, b);
        lhs.set_trit(i, c);
    }
}

fn mutate2_trytes<T, F>(lhs: &mut T, rhs: &T, f: F)
where
    T: Ternary + ?Sized,
    F: Fn(Tryte, Tryte) -> Tryte,
{
    for i in 0..rhs.tryte_len() {
        let a = lhs.get_tryte(i);
        let b = rhs.get_tryte(i);
        let c = f(a, b);
        lhs.set_tryte(i, c);
    }
}

impl Ternary for [Tryte] {
    fn trit_len(&self) -> usize {
        self.tryte_len() * tryte::TRIT_LEN
    }

    fn tryte_len(&self) -> usize {
        self.len()
    }

    fn get_trit(&self, i: usize) -> Trit {
        let (tryte_index, trit_index) = indices(i);
        let tryte = self[tryte_index];
        tryte.get_trit(trit_index)
    }

    fn set_trit(&mut self, i: usize, trit: Trit) {
        let (tryte_index, trit_index) = indices(i);
        let tryte = self[tryte_index];
        self[tryte_index] = tryte.set_trit(trit_index, trit);
    }

    fn get_tryte(&self, i: usize) -> Tryte {
        self[i]
    }

    fn set_tryte(&mut self, i: usize, tryte: Tryte) {
        self[i] = tryte;
    }
}

fn indices(i: usize) -> (usize, usize) {
    let tryte_index = i / tryte::TRIT_LEN;
    let trit_index = i % tryte::TRIT_LEN;
    (tryte_index, trit_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_constants::*;

    use std::io::Cursor;

    #[test]
    fn ternary_into_i64() {
        assert_eq!(WORD_MIN, TRYTE4_MIN.into_i64());
        assert_eq!(-1i64, TRYTE4_NEG1.into_i64());
        assert_eq!(0i64, TRYTE4_0.into_i64());
        assert_eq!(1i64, TRYTE4_1.into_i64());
        assert_eq!(WORD_MAX, TRYTE4_MAX.into_i64());
    }

    #[test]
    fn ternary_read_i64() {
        assert_eq!(&TRYTE4_MIN, &tryte4_from_int(WORD_MIN).unwrap()[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_from_int(-1).unwrap()[..]);
        assert_eq!(&TRYTE4_0, &tryte4_from_int(0).unwrap()[..]);
        assert_eq!(&TRYTE4_1, &tryte4_from_int(1).unwrap()[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_from_int(WORD_MAX).unwrap()[..]);

        assert!(tryte4_from_int(i64::min_value()).is_err());
        assert!(tryte4_from_int(i64::max_value()).is_err());
    }

    fn tryte4_from_int(n: i64) -> Result<Vec<Tryte>> {
        try_with_cloned_trytes(&TRYTE4_0, |ternary| ternary.read_i64(n))
    }

    #[test]
    fn ternary_read_bytes() {
        assert_eq!(&TRYTE4_MIN, &tryte4_from_bytes(&BYTES_MIN).unwrap()[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_from_bytes(&BYTES_NEG1).unwrap()[..]);
        assert_eq!(&TRYTE4_0, &tryte4_from_bytes(&BYTES_0).unwrap()[..]);
        assert_eq!(&TRYTE4_1, &tryte4_from_bytes(&BYTES_1).unwrap()[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_from_bytes(&BYTES_MAX).unwrap()[..]);
    }

    fn tryte4_from_bytes(bytes: &[u8]) -> Result<Vec<Tryte>> {
        try_with_cloned_trytes(&TRYTE4_0, |ternary| {
            ternary.read_bytes(&mut Cursor::new(bytes))
        })
    }

    #[test]
    fn ternary_write_bytes() {
        assert_eq!(&BYTES_MIN, &get_bytes(&TRYTE4_MIN[..])[..]);
        assert_eq!(&BYTES_NEG1, &get_bytes(&TRYTE4_NEG1[..])[..]);
        assert_eq!(&BYTES_0, &get_bytes(&TRYTE4_0[..])[..]);
        assert_eq!(&BYTES_1, &get_bytes(&TRYTE4_1[..])[..]);
        assert_eq!(&BYTES_MAX, &get_bytes(&TRYTE4_MAX[..])[..]);
    }

    fn get_bytes(trytes: &[Tryte]) -> Vec<u8> {
        let mut bytes = vec![];
        trytes.write_bytes(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn ternary_read_hytes() {
        assert_eq!(&TRYTE4_MIN, &tryte4_from_hyte_str("mmmmmmmm").unwrap()[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_from_hyte_str("0000000a").unwrap()[..]);
        assert_eq!(&TRYTE4_0, &tryte4_from_hyte_str("00000000").unwrap()[..]);
        assert_eq!(&TRYTE4_1, &tryte4_from_hyte_str("0000000A").unwrap()[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_from_hyte_str("MMMMMMMM").unwrap()[..]);
    }

    fn tryte4_from_hyte_str(s: &str) -> Result<Vec<Tryte>> {
        try_with_cloned_trytes(&TRYTE4_0, |ternary| ternary.read_hytes(s))
    }

    #[test]
    fn ternary_display_hytes() {
        assert_eq!("mmmmmmmm", get_hyte_str(&TRYTE4_MIN[..]));
        assert_eq!("0000000a", get_hyte_str(&TRYTE4_NEG1[..]));
        assert_eq!("00000000", get_hyte_str(&TRYTE4_0[..]));
        assert_eq!("0000000A", get_hyte_str(&TRYTE4_1[..]));
        assert_eq!("MMMMMMMM", get_hyte_str(&TRYTE4_MAX[..]));
    }

    fn get_hyte_str(trytes: &[Tryte]) -> String {
        let mut bytes = Vec::new();
        trytes.write_hytes(&mut bytes).unwrap();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    #[test]
    fn ternary_read_trits() {
        assert_eq!(
            &TRYTE4_MIN,
            &tryte4_from_trit_str("TTTTTTTTTTTTTTTTTTTTTTTT").unwrap()[..]
        );
        assert_eq!(
            &TRYTE4_NEG1,
            &tryte4_from_trit_str("00000000000000000000000T").unwrap()[..]
        );
        assert_eq!(
            &TRYTE4_0,
            &tryte4_from_trit_str("000000000000000000000000").unwrap()[..]
        );
        assert_eq!(
            &TRYTE4_1,
            &tryte4_from_trit_str("000000000000000000000001").unwrap()[..]
        );
        assert_eq!(
            &TRYTE4_MAX,
            &tryte4_from_trit_str("111111111111111111111111").unwrap()[..]
        );
    }

    fn tryte4_from_trit_str(s: &str) -> Result<Vec<Tryte>> {
        try_with_cloned_trytes(&TRYTE4_0, |ternary| ternary.read_trits(s))
    }

    #[test]
    fn ternary_display_trits() {
        assert_eq!("TTTTTTTTTTTTTTTTTTTTTTTT", get_trit_str(&TRYTE4_MIN[..]));
        assert_eq!("00000000000000000000000T", get_trit_str(&TRYTE4_NEG1[..]));
        assert_eq!("000000000000000000000000", get_trit_str(&TRYTE4_0[..]));
        assert_eq!("000000000000000000000001", get_trit_str(&TRYTE4_1[..]));
        assert_eq!("111111111111111111111111", get_trit_str(&TRYTE4_MAX[..]));
    }

    fn get_trit_str(trytes: &[Tryte]) -> String {
        let mut bytes = Vec::new();
        trytes.write_trits(&mut bytes).unwrap();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    #[test]
    fn ternary_cmp() {
        assert_eq!(trit::ZERO, compare(&TRYTE4_0[..], &TRYTE4_0[..]));
        assert_eq!(trit::NEG, compare(&TRYTE4_0[..], &TRYTE4_MAX[..]));
        assert_eq!(trit::POS, compare(&TRYTE4_0[..], &TRYTE4_MIN[..]));
        assert_eq!(trit::POS, compare(&TRYTE4_MAX[..], &TRYTE4_0[..]));
        assert_eq!(trit::POS, compare(&TRYTE4_MAX[..], &TRYTE4_MIN[..]));
        assert_eq!(trit::ZERO, compare(&TRYTE4_MAX[..], &TRYTE4_MAX[..]));
        assert_eq!(trit::NEG, compare(&TRYTE4_MIN[..], &TRYTE4_0[..]));
        assert_eq!(trit::NEG, compare(&TRYTE4_MIN[..], &TRYTE4_MAX[..]));
        assert_eq!(trit::ZERO, compare(&TRYTE4_MIN[..], &TRYTE4_MIN[..]));
    }

    #[test]
    fn ternary_negate() {
        assert_eq!(&TRYTE4_MIN, &tryte4_negate(&TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_negate(&TRYTE4_1)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_negate(&TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_negate(&TRYTE4_NEG1)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_negate(&TRYTE4_MIN)[..]);
    }

    fn tryte4_negate(trytes: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes2(&TRYTE4_0, trytes, negate)
    }

    #[test]
    fn ternary_and() {
        assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_0, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_0, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_0, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_and(&TRYTE4_MAX, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_and(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_and(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
    }

    fn tryte4_and(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes3(&TRYTE4_0, trytes1, trytes2, and)
    }

    #[test]
    fn ternary_or() {
        assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_0, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_0, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_0, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_or(&TRYTE4_MIN, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_or(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_or(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
    }

    fn tryte4_or(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes3(&TRYTE4_0, trytes1, trytes2, or)
    }

    #[test]
    fn ternary_tcmp() {
        assert_eq!(&TRYTE4_MIN, &tryte4_tcmp(&TRYTE4_MIN, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_tcmp(&TRYTE4_MAX, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_tcmp(&TRYTE4_NEG1, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_tcmp(&TRYTE4_1, &TRYTE4_0)[..]);

        assert_eq!(&TRYTE4_MAX, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_NEG1)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_tcmp(&TRYTE4_0, &TRYTE4_1)[..]);

        assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_NEG1, &TRYTE4_NEG1)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tcmp(&TRYTE4_1, &TRYTE4_1)[..]);
    }

    fn tryte4_tcmp(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes3(&TRYTE4_0, trytes1, trytes2, tcmp)
    }

    #[test]
    fn ternary_tmul() {
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_1, &TRYTE4_0)[..]);

        assert_eq!(&TRYTE4_MIN, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_MAX, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_MAX)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_tmul(&TRYTE4_1, &TRYTE4_MAX)[..]);

        assert_eq!(&TRYTE4_MAX, &tryte4_tmul(&TRYTE4_MIN, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_MIN, &tryte4_tmul(&TRYTE4_MAX, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_tmul(&TRYTE4_NEG1, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_tmul(&TRYTE4_0, &TRYTE4_MIN)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_tmul(&TRYTE4_1, &TRYTE4_MIN)[..]);
    }

    fn tryte4_tmul(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes3(&TRYTE4_0, trytes1, trytes2, tmul)
    }

    #[test]
    fn ternary_multiply() {
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_1, &TRYTE2_0)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_4096, &TRYTE2_0)[..]);

        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_NEG4096)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_NEG1)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_1)[..]);
        assert_eq!(&TRYTE4_0, &tryte4_mul(&TRYTE2_0, &TRYTE2_4096)[..]);

        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_1)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_1)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_mul(&TRYTE2_1, &TRYTE2_1)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_4096, &TRYTE2_1)[..]);

        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_1, &TRYTE2_NEG4096)[..]);
        assert_eq!(&TRYTE4_NEG1, &tryte4_mul(&TRYTE2_1, &TRYTE2_NEG1)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_1, &TRYTE2_4096)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG4096, &TRYTE2_NEG1)[..]);
        assert_eq!(&TRYTE4_1, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_NEG1)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_4096, &TRYTE2_NEG1)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_NEG4096)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG1, &TRYTE2_4096)[..]);

        assert_eq!(&TRYTE4_64, &tryte4_mul(&TRYTE2_8, &TRYTE2_8)[..]);
        assert_eq!(&TRYTE4_64, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_NEG8)[..]);
        assert_eq!(&TRYTE4_NEG64, &tryte4_mul(&TRYTE2_8, &TRYTE2_NEG8)[..]);
        assert_eq!(&TRYTE4_NEG64, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_8)[..]);

        assert_eq!(&TRYTE4_81, &tryte4_mul(&TRYTE2_9, &TRYTE2_9)[..]);
        assert_eq!(&TRYTE4_81, &tryte4_mul(&TRYTE2_NEG9, &TRYTE2_NEG9)[..]);
        assert_eq!(&TRYTE4_NEG81, &tryte4_mul(&TRYTE2_9, &TRYTE2_NEG9)[..]);
        assert_eq!(&TRYTE4_NEG81, &tryte4_mul(&TRYTE2_NEG9, &TRYTE2_9)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_8, &TRYTE2_512)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_NEG512)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_8, &TRYTE2_NEG512)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG8, &TRYTE2_512)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_512, &TRYTE2_8)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG512, &TRYTE2_NEG8)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_512, &TRYTE2_NEG8)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG512, &TRYTE2_8)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_16, &TRYTE2_256)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG16, &TRYTE2_NEG256)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_16, &TRYTE2_NEG256)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG16, &TRYTE2_256)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_256, &TRYTE2_16)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG256, &TRYTE2_NEG16)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_256, &TRYTE2_NEG16)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG256, &TRYTE2_16)[..]);

        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_64, &TRYTE2_64)[..]);
        assert_eq!(&TRYTE4_4096, &tryte4_mul(&TRYTE2_NEG64, &TRYTE2_NEG64)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_64, &TRYTE2_NEG64)[..]);
        assert_eq!(&TRYTE4_NEG4096, &tryte4_mul(&TRYTE2_NEG64, &TRYTE2_64)[..]);
    }

    fn tryte4_mul(trytes1: &[Tryte], trytes2: &[Tryte]) -> Vec<Tryte> {
        with_cloned_trytes3(&TRYTE4_0, trytes1, trytes2, multiply)
    }

    #[test]
    fn ternary_shift() {
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000000000000000000001T000110T001100T011000T",
            ),
            tryte4_shift("1T000110T001100T011000T1", -25)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000000000000000000001T000110T001100T011000T1",
            ),
            tryte4_shift("1T000110T001100T011000T1", -24)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000000000000000000001T000110T001100T011000T10",
            ),
            tryte4_shift("1T000110T001100T011000T1", -23)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000000000000000001T000110T001100T011000T100",
            ),
            tryte4_shift("1T000110T001100T011000T1", -22)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000000000000000001T000110T001100T011000T1000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -21)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000000000000000001T000110T001100T011000T10000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -20)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000000000000001T000110T001100T011000T100000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -19)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000000000000001T000110T001100T011000T1000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -18)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000000000000001T000110T001100T011000T10000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -17)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000000000001T000110T001100T011000T100000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -16)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000000000001T000110T001100T011000T1000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -15)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000000000001T000110T001100T011000T10000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -14)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000000001T000110T001100T011000T100000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -13)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000000001T000110T001100T011000T1000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -12)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000000001T000110T001100T011000T10000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -11)
        );

        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000000001T000110T001100T011000T100000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -10)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000000001T000110T001100T011000T1000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -9)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000000001T000110T001100T011000T10000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -8)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000000001T000110T001100T011000T100000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -7)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000000001T000110T001100T011000T1000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -6)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000000001T000110T001100T011000T10000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -5)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000000001T000110T001100T011000T100000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -4)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000000001T000110T001100T011000T1000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -3)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000000001T000110T001100T011000T10000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -2)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000000001T000110T001100T011000T100000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", -1)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000000001T000110T001100T011000T1000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 0)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000000001T000110T001100T011000T10000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 1)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000000001T000110T001100T011000T100000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 2)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000000001T000110T001100T011000T1000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 3)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000000001T000110T001100T011000T10000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 4)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000000001T000110T001100T011000T100000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 5)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000000001T000110T001100T011000T1000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 6)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000000001T000110T001100T011000T10000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 7)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000000001T000110T001100T011000T100000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 8)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000000001T000110T001100T011000T1000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 9)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000000001T000110T001100T011000T10000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 10)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000000001T000110T001100T011000T100000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 11)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000000001T000110T001100T011000T1000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 12)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000000001T000110T001100T011000T10000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 13)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000000001T000110T001100T011000T100000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 14)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000000001T000110T001100T011000T1000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 15)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000000001T000110T001100T011000T10000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 16)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00000001T000110T001100T011000T100000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 17)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0000001T000110T001100T011000T1000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 18)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "000001T000110T001100T011000T10000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 19)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "00001T000110T001100T011000T100000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 20)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "0001T000110T001100T011000T1000000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 21)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "001T000110T001100T011000T10000000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 22)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "01T000110T001100T011000T100000000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 23)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "1T000110T001100T011000T1000000000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 24)
        );
        assert_eq!(
            tryte12_from_trit_str(
                "T000110T001100T011000T10000000000000000000000000000000000000000000000000",
            ),
            tryte4_shift("1T000110T001100T011000T1", 25)
        );
    }

    fn tryte4_shift(trit_str: &str, offset: isize) -> Result<Vec<Tryte>> {
        let trytes = tryte4_from_trit_str(trit_str)?;
        try_with_cloned_trytes2(&TRYTE12_0, &trytes[..], |dest, src| {
            shift(dest, src, offset);
            Ok(())
        })
    }

    fn tryte12_from_trit_str(s: &str) -> Result<Vec<Tryte>> {
        try_with_cloned_trytes(&TRYTE12_0, |ternary| ternary.read_trits(s))
    }

    fn clone_slice<T: Clone>(slice: &[T]) -> Vec<T> {
        let mut vec = Vec::new();
        vec.extend_from_slice(slice);
        vec
    }

    fn with_cloned_trytes<F>(trytes: &[Tryte], mut f: F) -> Vec<Tryte>
    where
        F: FnMut(&mut [Tryte]),
    {
        let mut trytes = clone_slice(trytes);
        f(&mut trytes[..]);
        trytes
    }

    fn with_cloned_trytes2<F>(trytes1: &[Tryte], trytes2: &[Tryte], mut f: F) -> Vec<Tryte>
    where
        F: FnMut(&mut [Tryte], &[Tryte]),
    {
        let mut trytes1 = clone_slice(trytes1);
        f(&mut trytes1[..], &trytes2);
        trytes1
    }

    fn with_cloned_trytes3<F>(
        trytes1: &[Tryte],
        trytes2: &[Tryte],
        trytes3: &[Tryte],
        mut f: F,
    ) -> Vec<Tryte>
    where
        F: FnMut(&mut [Tryte], &[Tryte], &[Tryte]),
    {
        let mut trytes1 = clone_slice(trytes1);
        f(&mut trytes1[..], &trytes2, &trytes3);
        trytes1
    }

    fn try_with_cloned_trytes<F>(trytes: &[Tryte], mut f: F) -> Result<Vec<Tryte>>
    where
        F: FnMut(&mut [Tryte]) -> Result<()>,
    {
        let mut trytes = clone_slice(trytes);
        f(&mut trytes[..])?;
        Ok(trytes)
    }

    fn try_with_cloned_trytes2<F>(
        trytes1: &[Tryte],
        trytes2: &[Tryte],
        mut f: F,
    ) -> Result<Vec<Tryte>>
    where
        F: FnMut(&mut [Tryte], &[Tryte]) -> Result<()>,
    {
        let mut trytes1 = clone_slice(trytes1);
        f(&mut trytes1[..], &trytes2)?;
        Ok(trytes1)
    }
}
