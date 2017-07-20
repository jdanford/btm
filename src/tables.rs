use std::u8;
use std::u16;

lazy_static! {
    pub static ref TRIT2_TO_AND: [u16; 16] = {
        let mut table = [u16::MAX; 16];

        table[0b00_00] = 0b00;
        table[0b00_01] = 0b00;
        table[0b00_11] = 0b11;
        table[0b01_00] = 0b00;
        table[0b01_01] = 0b01;
        table[0b01_11] = 0b11;
        table[0b11_00] = 0b11;
        table[0b11_01] = 0b11;
        table[0b11_11] = 0b11;

        table
    };

    pub static ref TRIT2_TO_CMP: [u16; 16] = {
        let mut table = [u16::MAX; 16];

        table[0b00_00] = 0b00;
        table[0b00_01] = 0b11;
        table[0b00_11] = 0b01;
        table[0b01_00] = 0b01;
        table[0b01_01] = 0b00;
        table[0b01_11] = 0b01;
        table[0b11_00] = 0b11;
        table[0b11_01] = 0b11;
        table[0b11_11] = 0b00;

        table
    };

    pub static ref TRIT2_TO_OR: [u16; 16] = {
        let mut table = [u16::MAX; 16];

        table[0b00_00] = 0b00;
        table[0b00_01] = 0b01;
        table[0b00_11] = 0b00;
        table[0b01_00] = 0b01;
        table[0b01_01] = 0b01;
        table[0b01_11] = 0b01;
        table[0b11_00] = 0b00;
        table[0b11_01] = 0b01;
        table[0b11_11] = 0b11;

        table
    };

    pub static ref TRIT2_TO_PRODUCT: [u16; 16] = {
        let mut table = [u16::MAX; 16];

        table[0b00_00] = 0b00;
        table[0b00_01] = 0b00;
        table[0b00_11] = 0b00;
        table[0b01_00] = 0b00;
        table[0b01_01] = 0b01;
        table[0b01_11] = 0b11;
        table[0b11_00] = 0b00;
        table[0b11_01] = 0b11;
        table[0b11_11] = 0b01;

        table
    };

    pub static ref TRIT3_TO_SUM_AND_CARRY: [(u16, u16); 64] = {
        let mut table = [(u16::MAX, u16::MAX); 64];

        table[0b00_00_00] = (0b00, 0b00);
        table[0b00_00_01] = (0b01, 0b00);
        table[0b00_00_11] = (0b11, 0b00);
        table[0b00_01_00] = (0b01, 0b00);
        table[0b00_01_01] = (0b11, 0b01);
        table[0b00_01_11] = (0b00, 0b00);
        table[0b00_11_00] = (0b11, 0b00);
        table[0b00_11_01] = (0b00, 0b00);
        table[0b00_11_11] = (0b01, 0b11);
        table[0b01_00_00] = (0b01, 0b00);
        table[0b01_00_01] = (0b11, 0b01);
        table[0b01_00_11] = (0b00, 0b00);
        table[0b01_01_00] = (0b11, 0b01);
        table[0b01_01_01] = (0b00, 0b01);
        table[0b01_01_11] = (0b01, 0b00);
        table[0b01_11_00] = (0b00, 0b00);
        table[0b01_11_01] = (0b01, 0b00);
        table[0b01_11_11] = (0b11, 0b00);
        table[0b11_00_00] = (0b11, 0b00);
        table[0b11_00_01] = (0b01, 0b00);
        table[0b11_00_11] = (0b01, 0b11);
        table[0b11_01_00] = (0b00, 0b00);
        table[0b11_01_01] = (0b01, 0b00);
        table[0b11_01_11] = (0b11, 0b00);
        table[0b11_11_00] = (0b01, 0b11);
        table[0b11_11_01] = (0b11, 0b00);
        table[0b11_11_11] = (0b00, 0b11);

        table
    };

    pub static ref TRIT4_TO_U8: [u8; 256] = {
        let mut table = [u8::MAX; 256];

        table[0b00_00_00_00] = 0;
        table[0b00_00_00_01] = 1;
        table[0b00_00_01_11] = 2;
        table[0b00_00_01_00] = 3;
        table[0b00_00_01_01] = 4;
        table[0b00_01_11_11] = 5;
        table[0b00_01_11_00] = 6;
        table[0b00_01_11_01] = 7;
        table[0b00_01_00_11] = 8;
        table[0b00_01_00_00] = 9;
        table[0b00_01_00_01] = 10;
        table[0b00_01_01_11] = 11;
        table[0b00_01_01_00] = 12;
        table[0b00_01_01_01] = 13;
        table[0b01_11_11_11] = 14;
        table[0b01_11_11_00] = 15;
        table[0b01_11_11_01] = 16;
        table[0b01_11_00_11] = 17;
        table[0b01_11_00_00] = 18;
        table[0b01_11_00_01] = 19;
        table[0b01_11_01_11] = 20;
        table[0b01_11_01_00] = 21;
        table[0b01_11_01_01] = 22;
        table[0b01_00_11_11] = 23;
        table[0b01_00_11_00] = 24;
        table[0b01_00_11_01] = 25;
        table[0b01_00_00_11] = 26;
        table[0b01_00_00_00] = 27;
        table[0b01_00_00_01] = 28;
        table[0b01_00_01_11] = 29;
        table[0b01_00_01_00] = 30;
        table[0b01_00_01_01] = 31;
        table[0b01_01_11_11] = 32;
        table[0b01_01_11_00] = 33;
        table[0b01_01_11_01] = 34;
        table[0b01_01_00_11] = 35;

        table
    };
}

pub static U8_TO_TRIT4: [u8; 36] = [
    0b00_00_00_00,
    0b00_00_00_01,
    0b00_00_01_11,
    0b00_00_01_00,
    0b00_00_01_01,
    0b00_01_11_11,
    0b00_01_11_00,
    0b00_01_11_01,
    0b00_01_00_11,
    0b00_01_00_00,
    0b00_01_00_01,
    0b00_01_01_11,
    0b00_01_01_00,
    0b00_01_01_01,
    0b01_11_11_11,
    0b01_11_11_00,
    0b01_11_11_01,
    0b01_11_00_11,
    0b01_11_00_00,
    0b01_11_00_01,
    0b01_11_01_11,
    0b01_11_01_00,
    0b01_11_01_01,
    0b01_00_11_11,
    0b01_00_11_00,
    0b01_00_11_01,
    0b01_00_00_11,
    0b01_00_00_00,
    0b01_00_00_01,
    0b01_00_01_11,
    0b01_00_01_00,
    0b01_00_01_01,
    0b01_01_11_11,
    0b01_01_11_00,
    0b01_01_11_01,
    0b01_01_00_11,
];
