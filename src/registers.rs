#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StandardRegister(pub u8);

pub const ZERO: StandardRegister = StandardRegister(0);
pub const LO: StandardRegister = StandardRegister(1);
pub const HI: StandardRegister = StandardRegister(2);
pub const SP: StandardRegister = StandardRegister(3);
pub const FP: StandardRegister = StandardRegister(4);
pub const RA: StandardRegister = StandardRegister(5);
pub const A0: StandardRegister = StandardRegister(6);
pub const A1: StandardRegister = StandardRegister(7);
pub const A2: StandardRegister = StandardRegister(8);
pub const A3: StandardRegister = StandardRegister(9);
pub const A4: StandardRegister = StandardRegister(10);
pub const A5: StandardRegister = StandardRegister(11);
pub const S0: StandardRegister = StandardRegister(12);
pub const S1: StandardRegister = StandardRegister(13);
pub const S2: StandardRegister = StandardRegister(14);
pub const S3: StandardRegister = StandardRegister(15);
pub const S4: StandardRegister = StandardRegister(16);
pub const S5: StandardRegister = StandardRegister(17);
pub const T0: StandardRegister = StandardRegister(18);
pub const T1: StandardRegister = StandardRegister(19);
pub const T2: StandardRegister = StandardRegister(20);
pub const T3: StandardRegister = StandardRegister(21);
pub const T4: StandardRegister = StandardRegister(22);
pub const T5: StandardRegister = StandardRegister(23);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SystemRegister(pub u8);

pub const EHA: SystemRegister = SystemRegister(0);
pub const ERA: SystemRegister = SystemRegister(1);
pub const EC: SystemRegister = SystemRegister(2);
pub const ED: SystemRegister = SystemRegister(3);
