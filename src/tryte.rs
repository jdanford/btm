use std::convert::TryFrom;
use std::ops::Neg;

use trit::Trit;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
        let bits = (self.0 | (trit.0 << shf)) & Tryte::BITMASK;
        Tryte(bits)
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
