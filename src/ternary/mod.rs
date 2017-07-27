pub mod error;
pub mod constants;
pub mod tables;
pub mod trit;
mod hyte;
pub mod tryte;

use std::convert::TryFrom;
use std::fmt;
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

    fn write_hytes<W: fmt::Write>(&self, writer: &mut W) -> Result<()> {
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

    fn write_trits<W: fmt::Write>(&self, writer: &mut W) -> Result<()> {
        for i in (0..self.trit_len()).rev() {
            let trit = self.get_trit(i);
            let c: char = trit.into();
            write!(writer, "{}", c)?;
        }

        Ok(())
    }

    fn compare(&self, rhs: &Self) -> Trit {
        let mut cmp_trit = trit::ZERO;

        for i in (0..self.trit_len()).rev() {
            let a = self.get_trit(i);
            let b = rhs.get_trit(i);
            cmp_trit = a.tcmp(b);

            if cmp_trit != trit::ZERO {
                break;
            }
        }

        cmp_trit
    }
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
