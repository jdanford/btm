use tryte::Tryte;

pub const TRYTE_MIN: Tryte = Tryte(0b11_11_11_11_11_11);
pub const TRYTE_NEG64: Tryte = Tryte(0b00_11_01_11_00_11);
pub const TRYTE_NEG1: Tryte = Tryte(0b00_00_00_00_00_11);
pub const TRYTE_0: Tryte = Tryte(0b00_00_00_00_00_00);
pub const TRYTE_1: Tryte = Tryte(0b00_00_00_00_00_01);
pub const TRYTE_64: Tryte = Tryte(0b00_01_11_01_00_01);
pub const TRYTE_MAX: Tryte = Tryte(0b01_01_01_01_01_01);
