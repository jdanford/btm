use std::convert::TryFrom;
use std::ops::Neg;

use phf;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Trit(pub u16);

impl Trit {
    pub const BITMASK: u16 = 0b11;

    pub const BITS_ZERO: u16 = 0b00;
    pub const BITS_POS: u16 = 0b01;
    pub const BITS_NEG: u16 = 0b10;

    pub const CHAR_ZERO: char = '0';
    pub const CHAR_POS: char = '1';
    pub const CHAR_NEG: char = 'T';

    pub const ZERO: Trit = Trit(Trit::BITS_ZERO);
    pub const POS: Trit = Trit(Trit::BITS_POS);
    pub const NEG: Trit = Trit(Trit::BITS_NEG);
}

static BITS_TO_CHAR: [char; 3] = [Trit::CHAR_ZERO, Trit::CHAR_POS, Trit::CHAR_NEG];

impl Into<char> for Trit {
    fn into(self) -> char {
        BITS_TO_CHAR[self.0 as usize]
    }
}

static CHAR_TO_BITS: phf::Map<char, u16> = phf_map! {
    'T' => Trit::BITS_NEG,
    '0' => Trit::BITS_ZERO,
    '1' => Trit::BITS_POS,
};

impl TryFrom<char> for Trit {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if let Some(&bits) = CHAR_TO_BITS.get(&c) {
            Ok(Trit(bits))
        } else {
            Err(())
        }
    }
}

static BITS_TO_INT: [i16; 3] = [0, 1, -1];

impl Into<i16> for Trit {
    fn into(self) -> i16 {
        BITS_TO_INT[self.0 as usize]
    }
}

static SINT_TO_BITS: [u16; 3] = [Trit::BITS_NEG, Trit::BITS_ZERO, Trit::BITS_POS];

impl TryFrom<i16> for Trit {
    type Error = ();

    fn try_from(n: i16) -> Result<Self, Self::Error> {
        let sint = (n + 1) as usize;
        if sint < 3 {
            let bits = SINT_TO_BITS[sint];
            Ok(Trit(bits))
        } else {
            Err(())
        }
    }
}

impl Neg for Trit {
    type Output = Trit;

    fn neg(self) -> Self::Output {
        Trit(self.0 ^ Trit::BITMASK)
    }
}
