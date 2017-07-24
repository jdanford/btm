use tables::TRIT4_TO_USIZE;
use error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Opcode(usize);

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
        let i = TRIT4_TO_USIZE[trit4 as usize];
        if i > BREAK.0 {
            return Err(Error::InvalidOpcode(i));
        }

        Ok(Opcode(i))
    }
}
