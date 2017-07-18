use error::{Error, Result};
use trit;
use trit::Trit;
use tryte;
use tryte::Tryte;

pub const WORD_MIN: i64 = -141_214_768_240;
pub const WORD_MAX: i64 = 141_214_768_240;

#[derive(Debug, Eq, PartialEq)]
pub struct Ternary<'a> {
    pub trytes: &'a mut [Tryte],
}

impl<'a> Ternary<'a> {
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

fn indices(i: usize) -> (usize, usize) {
    let tryte_index = i / tryte::TRIT_LEN;
    let trit_index = i % tryte::TRIT_LEN;
    (tryte_index, trit_index)
}
