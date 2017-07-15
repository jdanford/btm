use std::convert::{TryFrom, TryInto};
use std::cmp::Ordering;
use std::ops;
use std::ops::Mul;

use trit;
use trit::Trit;

pub const MIN_VALUE: i16 = -364;
pub const MAX_VALUE: i16 = 364;

const BITMASK: u16 = 0b11_11_11_11_11_11;
const SIGN_BITMASK: u16 = 0b10_10_10_10_10_10;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq)]
pub struct Tryte(pub u16);

pub const ZERO: Tryte = Tryte(trit::BIN_ZERO);

impl Tryte {
    pub fn get_trit(self, i: usize) -> Trit {
        let shf = (i as u16) * 2;
        let bits = self.0 >> shf & trit::BITMASK;
        Trit(bits)
    }

    pub fn set_trit(self, i: usize, trit: Trit) -> Self {
        let shf = (i as u16) * 2;
        let bits = (self.0 | trit.0 << shf) & BITMASK;
        Tryte(bits)
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
    type Error = ();

    fn try_into(self) -> Result<Trit, Self::Error> {
        let bits = self.0;
        if bits == trit::BIN_INVALID || bits > trit::BIN_NEG {
            Err(())
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
    type Error = ();

    fn try_from(n: i16) -> Result<Self, Self::Error> {
        if n < MIN_VALUE || MAX_VALUE < n {
            return Err(());
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
