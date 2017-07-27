use ternary::Tryte;

pub const TRYTE_MIN: Tryte = Tryte(0b11_11_11_11_11_11);
pub const TRYTE_NEG317: Tryte = Tryte(0b11_01_00_01_11_01);
pub const TRYTE_NEG278: Tryte = Tryte(0b11_00_11_11_00_01);
pub const TRYTE_NEG256: Tryte = Tryte(0b11_00_00_11_11_11);
pub const TRYTE_NEG217: Tryte = Tryte(0b11_00_01_00_00_11);
pub const TRYTE_NEG167: Tryte = Tryte(0b11_01_00_11_01_01);
pub const TRYTE_NEG105: Tryte = Tryte(0b00_11_11_00_01_00);
pub const TRYTE_NEG81: Tryte = Tryte(0b00_11_00_00_00_00);
pub const TRYTE_NEG64: Tryte = Tryte(0b00_11_01_11_00_11);
pub const TRYTE_NEG16: Tryte = Tryte(0b00_00_11_01_01_11);
pub const TRYTE_NEG9: Tryte = Tryte(0b00_00_00_11_00_00);
pub const TRYTE_NEG8: Tryte = Tryte(0b00_00_00_11_00_01);
pub const TRYTE_NEG6: Tryte = Tryte(0b00_00_00_11_01_00);
pub const TRYTE_NEG3: Tryte = Tryte(0b00_00_00_00_11_00);
pub const TRYTE_NEG1: Tryte = Tryte(0b00_00_00_00_00_11);
pub const TRYTE_0: Tryte = Tryte(0b00_00_00_00_00_00);
pub const TRYTE_1: Tryte = Tryte(0b00_00_00_00_00_01);
pub const TRYTE_3: Tryte = Tryte(0b00_00_00_00_01_00);
pub const TRYTE_6: Tryte = Tryte(0b00_00_00_01_11_00);
pub const TRYTE_8: Tryte = Tryte(0b00_00_00_01_00_11);
pub const TRYTE_9: Tryte = Tryte(0b00_00_00_01_00_00);
pub const TRYTE_16: Tryte = Tryte(0b00_00_01_11_11_01);
pub const TRYTE_64: Tryte = Tryte(0b00_01_11_01_00_01);
pub const TRYTE_81: Tryte = Tryte(0b00_01_00_00_00_00);
pub const TRYTE_105: Tryte = Tryte(0b00_01_01_00_11_00);
pub const TRYTE_167: Tryte = Tryte(0b01_11_00_01_11_11);
pub const TRYTE_217: Tryte = Tryte(0b01_00_11_00_00_01);
pub const TRYTE_256: Tryte = Tryte(0b01_00_00_01_01_01);
pub const TRYTE_278: Tryte = Tryte(0b01_00_01_01_00_11);
pub const TRYTE_317: Tryte = Tryte(0b01_01_00_11_01_11);
pub const TRYTE_MAX: Tryte = Tryte(0b01_01_01_01_01_01);

pub const TRYTE2_NEG4096: [Tryte; 2] = [TRYTE_278, TRYTE_NEG6];
pub const TRYTE2_NEG512: [Tryte; 2] = [TRYTE_217, TRYTE_NEG1];
pub const TRYTE2_NEG256: [Tryte; 2] = [TRYTE_NEG256, TRYTE_0];
pub const TRYTE2_NEG64: [Tryte; 2] = [TRYTE_NEG64, TRYTE_0];
pub const TRYTE2_NEG16: [Tryte; 2] = [TRYTE_NEG16, TRYTE_0];
pub const TRYTE2_NEG9: [Tryte; 2] = [TRYTE_NEG9, TRYTE_0];
pub const TRYTE2_NEG8: [Tryte; 2] = [TRYTE_NEG8, TRYTE_0];
pub const TRYTE2_NEG1: [Tryte; 2] = [TRYTE_NEG1, TRYTE_0];
pub const TRYTE2_0: [Tryte; 2] = [TRYTE_0, TRYTE_0];
pub const TRYTE2_1: [Tryte; 2] = [TRYTE_1, TRYTE_0];
pub const TRYTE2_8: [Tryte; 2] = [TRYTE_8, TRYTE_0];
pub const TRYTE2_9: [Tryte; 2] = [TRYTE_9, TRYTE_0];
pub const TRYTE2_16: [Tryte; 2] = [TRYTE_16, TRYTE_0];
pub const TRYTE2_64: [Tryte; 2] = [TRYTE_64, TRYTE_0];
pub const TRYTE2_256: [Tryte; 2] = [TRYTE_256, TRYTE_0];
pub const TRYTE2_512: [Tryte; 2] = [TRYTE_NEG217, TRYTE_1];
pub const TRYTE2_4096: [Tryte; 2] = [TRYTE_NEG278, TRYTE_6];

pub const TRYTE4_MIN: [Tryte; 4] = [TRYTE_MIN; 4];
pub const TRYTE4_NEG1073741808: [Tryte; 4] = [TRYTE_105, TRYTE_NEG317, TRYTE_167, TRYTE_NEG3];
pub const TRYTE4_NEG4096: [Tryte; 4] = [TRYTE_278, TRYTE_NEG6, TRYTE_0, TRYTE_0];
pub const TRYTE4_NEG81: [Tryte; 4] = [TRYTE_NEG81, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_NEG64: [Tryte; 4] = [TRYTE_NEG64, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_NEG1: [Tryte; 4] = [TRYTE_NEG1, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_0: [Tryte; 4] = [TRYTE_0; 4];
pub const TRYTE4_1: [Tryte; 4] = [TRYTE_1, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_64: [Tryte; 4] = [TRYTE_64, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_81: [Tryte; 4] = [TRYTE_81, TRYTE_0, TRYTE_0, TRYTE_0];
pub const TRYTE4_4096: [Tryte; 4] = [TRYTE_NEG278, TRYTE_6, TRYTE_0, TRYTE_0];
pub const TRYTE4_1073741808: [Tryte; 4] = [TRYTE_NEG105, TRYTE_317, TRYTE_NEG167, TRYTE_3];
pub const TRYTE4_MAX: [Tryte; 4] = [TRYTE_MAX; 4];

pub const TRYTE12_0: [Tryte; 12] = [TRYTE_0; 12];

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
