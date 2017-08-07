use error::{Error, Result};
use ternary::tables::TRIT4_TO_I8;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Opcode(u8);

pub const AND: Opcode = Opcode(0);
pub const OR: Opcode = Opcode(1);
pub const TMUL: Opcode = Opcode(2);
pub const TCMP: Opcode = Opcode(3);
pub const CMP: Opcode = Opcode(4);
pub const SHF: Opcode = Opcode(5);
pub const ADD: Opcode = Opcode(6);
pub const MUL: Opcode = Opcode(7);
pub const DIV: Opcode = Opcode(8);
pub const ANDI: Opcode = Opcode(9);
pub const ORI: Opcode = Opcode(10);
pub const TMULI: Opcode = Opcode(11);
pub const TCMPI: Opcode = Opcode(12);
pub const SHFI: Opcode = Opcode(13);
pub const ADDI: Opcode = Opcode(14);
pub const LUI: Opcode = Opcode(15);
pub const LSR: Opcode = Opcode(16);
pub const SSR: Opcode = Opcode(17);
pub const LT: Opcode = Opcode(18);
pub const LH: Opcode = Opcode(19);
pub const LW: Opcode = Opcode(20);
pub const ST: Opcode = Opcode(21);
pub const SH: Opcode = Opcode(22);
pub const SW: Opcode = Opcode(23);
pub const BT: Opcode = Opcode(24);
pub const B0: Opcode = Opcode(25);
pub const B1: Opcode = Opcode(26);
pub const BT0: Opcode = Opcode(27);
pub const BT1: Opcode = Opcode(28);
pub const B01: Opcode = Opcode(29);
pub const JMP: Opcode = Opcode(30);
pub const CALL: Opcode = Opcode(31);
pub const JMPR: Opcode = Opcode(32);
pub const CALLR: Opcode = Opcode(33);
pub const SYSCALL: Opcode = Opcode(34);
pub const BREAK: Opcode = Opcode(35);

impl Opcode {
    pub fn from_trit4(trit4: u8) -> Result<Opcode> {
        let i = TRIT4_TO_I8[trit4 as usize] as u8;
        if i > BREAK.0 {
            return Err(Error::InvalidOpcode(i));
        }

        Ok(Opcode(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_from_trit4() {
        assert_eq!(Ok(AND), Opcode::from_trit4(0b00_00_00_00));
        assert_eq!(Ok(OR), Opcode::from_trit4(0b00_00_00_01));
        assert_eq!(Ok(TMUL), Opcode::from_trit4(0b00_00_01_11));
        assert_eq!(Ok(TCMP), Opcode::from_trit4(0b00_00_01_00));
        assert_eq!(Ok(CMP), Opcode::from_trit4(0b00_00_01_01));
        assert_eq!(Ok(SHF), Opcode::from_trit4(0b00_01_11_11));
        assert_eq!(Ok(ADD), Opcode::from_trit4(0b00_01_11_00));
        assert_eq!(Ok(MUL), Opcode::from_trit4(0b00_01_11_01));
        assert_eq!(Ok(DIV), Opcode::from_trit4(0b00_01_00_11));
        assert_eq!(Ok(ANDI), Opcode::from_trit4(0b00_01_00_00));
        assert_eq!(Ok(ORI), Opcode::from_trit4(0b00_01_00_01));
        assert_eq!(Ok(TMULI), Opcode::from_trit4(0b00_01_01_11));
        assert_eq!(Ok(TCMPI), Opcode::from_trit4(0b00_01_01_00));
        assert_eq!(Ok(SHFI), Opcode::from_trit4(0b00_01_01_01));
        assert_eq!(Ok(ADDI), Opcode::from_trit4(0b01_11_11_11));
        assert_eq!(Ok(LUI), Opcode::from_trit4(0b01_11_11_00));
        assert_eq!(Ok(LSR), Opcode::from_trit4(0b01_11_11_01));
        assert_eq!(Ok(SSR), Opcode::from_trit4(0b01_11_00_11));
        assert_eq!(Ok(LT), Opcode::from_trit4(0b01_11_00_00));
        assert_eq!(Ok(LH), Opcode::from_trit4(0b01_11_00_01));
        assert_eq!(Ok(LW), Opcode::from_trit4(0b01_11_01_11));
        assert_eq!(Ok(ST), Opcode::from_trit4(0b01_11_01_00));
        assert_eq!(Ok(SH), Opcode::from_trit4(0b01_11_01_01));
        assert_eq!(Ok(SW), Opcode::from_trit4(0b01_00_11_11));
        assert_eq!(Ok(BT), Opcode::from_trit4(0b01_00_11_00));
        assert_eq!(Ok(B0), Opcode::from_trit4(0b01_00_11_01));
        assert_eq!(Ok(B1), Opcode::from_trit4(0b01_00_00_11));
        assert_eq!(Ok(BT0), Opcode::from_trit4(0b01_00_00_00));
        assert_eq!(Ok(BT1), Opcode::from_trit4(0b01_00_00_01));
        assert_eq!(Ok(B01), Opcode::from_trit4(0b01_00_01_11));
        assert_eq!(Ok(JMP), Opcode::from_trit4(0b01_00_01_00));
        assert_eq!(Ok(CALL), Opcode::from_trit4(0b01_00_01_01));
        assert_eq!(Ok(JMPR), Opcode::from_trit4(0b01_01_11_11));
        assert_eq!(Ok(CALLR), Opcode::from_trit4(0b01_01_11_00));
        assert_eq!(Ok(SYSCALL), Opcode::from_trit4(0b01_01_11_01));
        assert_eq!(Ok(BREAK), Opcode::from_trit4(0b01_01_00_11));

        assert!(Opcode::from_trit4(0b00_00_00_11).is_err());
        assert!(Opcode::from_trit4(0b01_01_00_00).is_err());
    }
}
