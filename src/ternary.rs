use std::convert::TryFrom;
use std::fmt;
use std::ops::{BitAnd, BitOr, Mul, Neg};

use byteorder::{ReadBytesExt, WriteBytesExt};

use error::{Error, Result};
use trit;
use trit::Trit;
use tryte;
use tryte::Tryte;

#[derive(Debug, Eq, PartialEq)]
pub struct Ternary<'a> {
    pub trytes: &'a mut [Tryte],
}

impl<'a> Ternary<'a> {
    pub fn new(trytes: &'a mut [Tryte]) -> Ternary<'a> {
        Ternary { trytes: trytes }
    }

    pub fn tryte_len(&self) -> usize {
        self.trytes.len()
    }

    pub fn trit_len(&self) -> usize {
        self.tryte_len() * tryte::TRIT_LEN
    }

    pub fn range(&self) -> i64 {
        let n = self.trit_len() as u32;
        3i64.pow(n)
    }

    pub fn min_value(&self) -> i64 {
        -(self.range() - 1) / 2
    }

    pub fn max_value(&self) -> i64 {
        (self.range() - 1) / 2
    }

    pub fn get_trit(&self, i: usize) -> Trit {
        let (tryte_index, trit_index) = indices(i);
        let tryte = self.trytes[tryte_index];
        tryte.get_trit(trit_index)
    }

    pub fn set_trit(&mut self, i: usize, trit: Trit) {
        let (tryte_index, trit_index) = indices(i);
        let tryte = self.trytes[tryte_index];
        self.trytes[tryte_index] = tryte.set_trit(trit_index, trit);
    }

    pub fn read_bytes<R: ReadBytesExt>(&mut self, reader: &mut R) -> Result<()> {
        for i in 0..self.tryte_len() {
            self.trytes[i] = Tryte::from_bytes(reader)?;
        }

        Ok(())
    }

    pub fn write_bytes<W: WriteBytesExt>(&self, writer: &mut W) -> Result<()> {
        for tryte in self.trytes.iter() {
            tryte.write_bytes(writer)?;
        }

        Ok(())
    }

    pub fn read_int(&mut self, n: i64) -> Result<()> {
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

    pub fn read_hyte_str(&mut self, s: &str) -> Result<()> {
        let len = self.tryte_len() * 2;
        if s.len() != len {
            return Err(Error::InvalidDataLength(len, s.len()));
        }

        let mut s = s;
        for i in (0..self.tryte_len()).rev() {
            let (substr, _s) = s.split_at(2);
            s = _s;
            let tryte = Tryte::from_hyte_str(substr)?;
            self.trytes[i] = tryte;
        }

        Ok(())
    }

    pub fn read_trit_str(&mut self, s: &str) -> Result<()> {
        if s.len() != self.trit_len() {
            return Err(Error::InvalidDataLength(self.trit_len(), s.len()));
        }

        for (i, c) in s.chars().rev().enumerate() {
            let trit = Trit::try_from(c)?;
            self.set_trit(i, trit);
        }

        Ok(())
    }

    fn read_trits(&mut self, trits: &[Trit]) -> Result<()> {
        if trits.len() != self.trit_len() {
            return Err(Error::InvalidDataLength(self.trit_len(), trits.len()));
        }

        for (i, &trit) in trits.iter().enumerate() {
            self.set_trit(i, trit);
        }

        Ok(())
    }

    pub fn display_hytes(&self) -> DisplayHytes {
        DisplayHytes(self)
    }

    pub fn display_trits(&self) -> DisplayTrits {
        DisplayTrits(self)
    }

    pub fn negate(&mut self) {
        self.mutate_trytes(Tryte::neg);
    }

    pub fn and(&mut self, rhs: &Ternary) {
        self.mutate2_trits(rhs, Trit::bitand);
    }

    pub fn or(&mut self, rhs: &Ternary) {
        self.mutate2_trits(rhs, Trit::bitor);
    }

    pub fn tcmp(&mut self, rhs: &Ternary) {
        self.mutate2_trits(rhs, Trit::tcmp)
    }

    pub fn tmul(&mut self, rhs: &Ternary) {
        self.mutate2_trits(rhs, Trit::mul)
    }

    pub fn add(&mut self, rhs: &Ternary, carry: Trit) -> Trit {
        let mut carry = carry;

        for i in 0..self.trit_len() {
            let a = self.get_trit(i);
            let b = rhs.get_trit(i);
            let (c, _carry) = a.add_with_carry(b, carry);
            carry = _carry;
            self.set_trit(i, c);
        }

        carry
    }

    pub fn compare(&self, rhs: &Ternary) -> Trit {
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

    fn mutate_trits<F: Fn(Trit) -> Trit>(&mut self, f: F) {
        for i in 0..self.trit_len() {
            let trit = self.get_trit(i);
            self.set_trit(i, f(trit));
        }
    }

    fn mutate_trytes<F: Fn(Tryte) -> Tryte>(&mut self, f: F) {
        for i in 0..self.tryte_len() {
            let tryte = self.trytes[i];
            self.trytes[i] = f(tryte);
        }
    }

    fn mutate2_trits<F: Fn(Trit, Trit) -> Trit>(&mut self, rhs: &Ternary, f: F) {
        for i in 0..self.trit_len() {
            let a = self.get_trit(i);
            let b = rhs.get_trit(i);
            let c = f(a, b);
            self.set_trit(i, c);
        }
    }

    fn mutate2_trytes<F: Fn(Tryte, Tryte) -> Tryte>(&mut self, rhs: &Ternary, f: F) {
        for i in 0..self.tryte_len() {
            let a = self.trytes[i];
            let b = rhs.trytes[i];
            let c = f(a, b);
            self.trytes[i] = c;
        }
    }
}

fn indices(i: usize) -> (usize, usize) {
    let tryte_index = i / tryte::TRIT_LEN;
    let trit_index = i % tryte::TRIT_LEN;
    (tryte_index, trit_index)
}

impl<'a> Into<i64> for Ternary<'a> {
    fn into(self) -> i64 {
        let mut n = 0i64;

        for i in (0..self.trit_len()).rev() {
            let trit = self.get_trit(i);
            let t: i16 = trit.into();
            n = n * 3 + t as i64;
        }

        n
    }
}

pub struct DisplayHytes<'a>(&'a Ternary<'a>);

impl<'a> fmt::Display for DisplayHytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0h")?;

        let ternary = self.0;
        for tryte in ternary.trytes.iter().rev() {
            tryte.fmt_hytes(f)?;
        }

        Ok(())
    }
}

pub struct DisplayTrits<'a>(&'a Ternary<'a>);

impl<'a> fmt::Display for DisplayTrits<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0t")?;

        let ternary = self.0;
        for i in (0..ternary.trit_len()).rev() {
            let trit = ternary.get_trit(i);
            let c: char = trit.into();
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}
