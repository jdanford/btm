use std::convert::TryFrom;
use std::ops::Neg;

use phf;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Trit(pub u16);

lazy_static! {
    static ref TRIT3_TO_SUM_AND_CARRY: [(u16, u16); 64] = {
        let mut table = [(0, 0); 64];

        table[0b00_00_00] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b00_00_01] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b00_00_11] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b00_01_00] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b00_01_01] = (Trit::BIN_NEG, Trit::BIN_POS);
        table[0b00_01_11] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b00_11_00] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b00_11_01] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b00_11_11] = (Trit::BIN_POS, Trit::BIN_NEG);
        table[0b01_00_00] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b01_00_01] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b01_00_11] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b01_01_00] = (Trit::BIN_NEG, Trit::BIN_POS);
        table[0b01_01_01] = (Trit::BIN_ZERO, Trit::BIN_POS);
        table[0b01_01_11] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b01_11_00] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b01_11_01] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b01_11_11] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b11_00_00] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b11_00_01] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b11_00_11] = (Trit::BIN_POS, Trit::BIN_NEG);
        table[0b11_01_00] = (Trit::BIN_ZERO, Trit::BIN_ZERO);
        table[0b11_01_01] = (Trit::BIN_POS, Trit::BIN_ZERO);
        table[0b11_01_11] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b11_11_00] = (Trit::BIN_POS, Trit::BIN_NEG);
        table[0b11_11_01] = (Trit::BIN_NEG, Trit::BIN_ZERO);
        table[0b11_11_11] = (Trit::BIN_ZERO, Trit::BIN_NEG);

        table
    };
}

impl Trit {
    pub const BITMASK: u16 = 0b11;

    pub const BIN_ZERO: u16 = 0b00;
    pub const BIN_POS: u16 = 0b01;
    pub const BIN_NEG: u16 = 0b11;

    pub const CHAR_ZERO: char = '0';
    pub const CHAR_POS: char = '1';
    pub const CHAR_NEG: char = 'T';

    pub const ZERO: Trit = Trit(Trit::BIN_ZERO);
    pub const POS: Trit = Trit(Trit::BIN_POS);
    pub const NEG: Trit = Trit(Trit::BIN_NEG);

    pub fn negation_bits(self) -> u16 {
        self.0 << 1 & Trit::BITMASK
    }

    pub fn add_with_carry(self, other: Trit, carry: Trit) -> (Trit, Trit) {
        let i = (self.0 << 4 | other.0 << 2 | carry.0) as usize;
        let (sum, carry) = TRIT3_TO_SUM_AND_CARRY[i];
        (Trit(sum), Trit(carry))
    }
}

static BITS_TO_INT: [i16; 4] = [0, 1, 0, -1];

impl Into<i16> for Trit {
    fn into(self) -> i16 {
        BITS_TO_INT[self.0 as usize]
    }
}

static UINT_TO_BITS: [u16; 3] = [Trit::BIN_NEG, Trit::BIN_ZERO, Trit::BIN_POS];

impl TryFrom<i16> for Trit {
    type Error = ();

    fn try_from(n: i16) -> Result<Self, Self::Error> {
        let uint = (n + 1) as usize;
        if uint < 3 {
            let bits = UINT_TO_BITS[uint];
            Ok(Trit(bits))
        } else {
            Err(())
        }
    }
}

static BITS_TO_CHAR: [char; 3] = [Trit::CHAR_ZERO, Trit::CHAR_POS, Trit::CHAR_NEG];

impl Into<char> for Trit {
    fn into(self) -> char {
        BITS_TO_CHAR[self.0 as usize]
    }
}

static CHAR_TO_BITS: phf::Map<char, u16> =
    phf_map! {
    'T' => Trit::BIN_NEG,
    '0' => Trit::BIN_ZERO,
    '1' => Trit::BIN_POS,
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

impl Neg for Trit {
    type Output = Trit;

    fn neg(self) -> Self::Output {
        let bits = self.0 ^ self.negation_bits();
        Trit(bits)
    }
}
