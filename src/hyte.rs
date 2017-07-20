use phf;

use trit::CHAR_INVALID;
use error::{Error, Result};

static CHAR_TO_HYTE: phf::Map<char, u8> =
    phf_map! {
    'm' => 0b11_11_11,
    'l' => 0b11_11_00,
    'k' => 0b11_11_01,
    'j' => 0b11_00_11,
    'i' => 0b11_00_00,
    'h' => 0b11_00_01,
    'g' => 0b11_01_11,
    'f' => 0b11_01_00,
    'e' => 0b11_01_01,
    'd' => 0b00_11_11,
    'c' => 0b00_11_00,
    'b' => 0b00_11_01,
    'a' => 0b00_00_11,
    '0' => 0b00_00_00,
    'A' => 0b00_00_01,
    'B' => 0b00_01_11,
    'C' => 0b00_01_00,
    'D' => 0b00_01_01,
    'E' => 0b01_11_11,
    'F' => 0b01_11_00,
    'G' => 0b01_11_01,
    'H' => 0b01_00_11,
    'I' => 0b01_00_00,
    'J' => 0b01_00_01,
    'K' => 0b01_01_11,
    'L' => 0b01_01_00,
    'M' => 0b01_01_01,
};

pub fn try_hyte_from_char(c: char) -> Result<u8> {
    CHAR_TO_HYTE.get(&c).cloned().ok_or_else(
        || Error::InvalidCharacter(c),
    )
}

lazy_static! {
    static ref HYTE_TO_CHAR: [char; 64] = {
        let mut table = [CHAR_INVALID; 64];

        table[0b11_11_11] = 'm';
        table[0b11_11_00] = 'l';
        table[0b11_11_01] = 'k';
        table[0b11_00_11] = 'j';
        table[0b11_00_00] = 'i';
        table[0b11_00_01] = 'h';
        table[0b11_01_11] = 'g';
        table[0b11_01_00] = 'f';
        table[0b11_01_01] = 'e';
        table[0b00_11_11] = 'd';
        table[0b00_11_00] = 'c';
        table[0b00_11_01] = 'b';
        table[0b00_00_11] = 'a';
        table[0b00_00_00] = '0';
        table[0b00_00_01] = 'A';
        table[0b00_01_11] = 'B';
        table[0b00_01_00] = 'C';
        table[0b00_01_01] = 'D';
        table[0b01_11_11] = 'E';
        table[0b01_11_00] = 'F';
        table[0b01_11_01] = 'G';
        table[0b01_00_11] = 'H';
        table[0b01_00_00] = 'I';
        table[0b01_00_01] = 'J';
        table[0b01_01_11] = 'K';
        table[0b01_01_00] = 'L';
        table[0b01_01_01] = 'M';

        table
    };
}

pub fn char_from_hyte(hyte: u8) -> char {
    let i = hyte as usize;
    HYTE_TO_CHAR[i]
}
