use std::convert::TryFrom;
use std::ops::Add;
use std::ops::Neg;

use trit::Trit;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tryte(pub u16);

impl Tryte {
    pub const BITMASK: u16 = 0b0000111111111111;

    pub const MIN_VALUE: i16 = -364;
    pub const MAX_VALUE: i16 = 364;

    pub fn get_trit(self, i: usize) -> Trit {
        let shf = (i as u16) * 2;
        let bits = (self.0 >> shf) & Trit::BITMASK;
        Trit(bits)
    }

    pub fn set_trit(self, i: usize, trit: Trit) -> Self {
        let shf = (i as u16) * 2;
        let bits = (self.0 | trit.0 << shf) & Tryte::BITMASK;
        Tryte(bits)
    }

    pub fn add_with_carry(self, other: Tryte, carry: Trit) -> (Tryte, Trit) {
        let mut carry = carry;
        let mut tryte = Tryte::default();

        for i in 0..6 {
            let a = self.get_trit(i);
            let b = other.get_trit(i);
            let (c, _carry) = a.add_with_carry(b, carry);
            carry = _carry;
            tryte = tryte.set_trit(i, c);
        }

        (tryte, carry)
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
        if n < Tryte::MIN_VALUE || Tryte::MAX_VALUE < n {
            return Err(())
        }

        let negative = n < 0;
        let mut n = n.abs();
        let mut tryte = Tryte(Trit::BITS_ZERO);

        for i in 0..6 {
            let rem_trit = match n % 3 {
                1 => Trit::POS,
                0 => Trit::ZERO,
                _ => {
                    n += 1;
                    Trit::NEG
                }
            };

            let trit = if negative { -rem_trit } else { rem_trit };
            tryte = tryte.set_trit(i, trit);
            n /= 3;
        }

        Ok(tryte)
    }
}

impl Neg for Tryte {
    type Output = Tryte;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ Tryte::BITMASK;
        Tryte(bits)
    }
}

impl Add for Tryte {
    type Output = Tryte;

    fn add(self, rhs: Tryte) -> Self::Output {
        let (sum, _) = self.add_with_carry(rhs, Trit::ZERO);
        sum
    }
}
