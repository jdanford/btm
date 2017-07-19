use tryte::Tryte;

pub const TRYTE_MIN: Tryte = Tryte(0b11_11_11_11_11_11);
pub const TRYTE_NEG64: Tryte = Tryte(0b00_11_01_11_00_11);
pub const TRYTE_NEG1: Tryte = Tryte(0b00_00_00_00_00_11);
pub const TRYTE_0: Tryte = Tryte(0b00_00_00_00_00_00);
pub const TRYTE_1: Tryte = Tryte(0b00_00_00_00_00_01);
pub const TRYTE_64: Tryte = Tryte(0b00_01_11_01_00_01);
pub const TRYTE_MAX: Tryte = Tryte(0b01_01_01_01_01_01);

pub const TRYTE4_MIN: [Tryte; 4] = [TRYTE_MIN, TRYTE_MIN, TRYTE_MIN, TRYTE_MIN];
pub const TRYTE4_NEG1: [Tryte; 4] = [TRYTE_NEG1, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_0: [Tryte; 4] = [TRYTE_0, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_1: [Tryte; 4] = [TRYTE_1, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_MAX: [Tryte; 4] = [TRYTE_MAX, TRYTE_MAX, TRYTE_MAX, TRYTE_MAX];

pub const BYTES_MIN: [u8; 8] = [
    0b11_11_11_11,
    0b00_00_11_11,
    0b11_11_11_11,
    0b00_00_11_11,
    0b11_11_11_11,
    0b00_00_11_11,
    0b11_11_11_11,
    0b00_00_11_11,
];

pub const BYTES_NEG1: [u8; 8] = [
    0b00_00_00_11,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
];

pub const BYTES_0: [u8; 8] = [
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
];

pub const BYTES_1: [u8; 8] = [
    0b00_00_00_01,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
    0b00_00_00_00,
];

pub const BYTES_MAX: [u8; 8] = [
    0b01_01_01_01,
    0b00_00_01_01,
    0b01_01_01_01,
    0b00_00_01_01,
    0b01_01_01_01,
    0b00_00_01_01,
    0b01_01_01_01,
    0b00_00_01_01,
];

pub const WORD_MIN: i64 = -141_214_768_240;
pub const WORD_MAX: i64 = 141_214_768_240;