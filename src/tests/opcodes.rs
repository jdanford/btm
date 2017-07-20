use std::convert::TryFrom;

use opcodes;
use opcodes::Opcode;

#[test]
fn opcode_from_trit4() {
    assert_eq!(Ok(opcodes::AND), Opcode::try_from(0b00_00_00_00));
    assert_eq!(Ok(opcodes::OR), Opcode::try_from(0b00_00_00_01));
    assert_eq!(Ok(opcodes::TMUL), Opcode::try_from(0b00_00_01_11));
    assert_eq!(Ok(opcodes::TCMP), Opcode::try_from(0b00_00_01_00));
    assert_eq!(Ok(opcodes::CMP), Opcode::try_from(0b00_00_01_01));
    assert_eq!(Ok(opcodes::SHF), Opcode::try_from(0b00_01_11_11));
    assert_eq!(Ok(opcodes::ADD), Opcode::try_from(0b00_01_11_00));
    assert_eq!(Ok(opcodes::MUL), Opcode::try_from(0b00_01_11_01));
    assert_eq!(Ok(opcodes::DIV), Opcode::try_from(0b00_01_00_11));
    assert_eq!(Ok(opcodes::ANDI), Opcode::try_from(0b00_01_00_00));
    assert_eq!(Ok(opcodes::ORI), Opcode::try_from(0b00_01_00_01));
    assert_eq!(Ok(opcodes::TMULI), Opcode::try_from(0b00_01_01_11));
    assert_eq!(Ok(opcodes::TCMPI), Opcode::try_from(0b00_01_01_00));
    assert_eq!(Ok(opcodes::SHFI), Opcode::try_from(0b00_01_01_01));
    assert_eq!(Ok(opcodes::ADDI), Opcode::try_from(0b01_11_11_11));
    assert_eq!(Ok(opcodes::LUI), Opcode::try_from(0b01_11_11_00));
    assert_eq!(Ok(opcodes::LSR), Opcode::try_from(0b01_11_11_01));
    assert_eq!(Ok(opcodes::SSR), Opcode::try_from(0b01_11_00_11));
    assert_eq!(Ok(opcodes::LT), Opcode::try_from(0b01_11_00_00));
    assert_eq!(Ok(opcodes::LH), Opcode::try_from(0b01_11_00_01));
    assert_eq!(Ok(opcodes::LW), Opcode::try_from(0b01_11_01_11));
    assert_eq!(Ok(opcodes::ST), Opcode::try_from(0b01_11_01_00));
    assert_eq!(Ok(opcodes::SH), Opcode::try_from(0b01_11_01_01));
    assert_eq!(Ok(opcodes::SW), Opcode::try_from(0b01_00_11_11));
    assert_eq!(Ok(opcodes::BT), Opcode::try_from(0b01_00_11_00));
    assert_eq!(Ok(opcodes::B0), Opcode::try_from(0b01_00_11_01));
    assert_eq!(Ok(opcodes::B1), Opcode::try_from(0b01_00_00_11));
    assert_eq!(Ok(opcodes::BT0), Opcode::try_from(0b01_00_00_00));
    assert_eq!(Ok(opcodes::BT1), Opcode::try_from(0b01_00_00_01));
    assert_eq!(Ok(opcodes::B01), Opcode::try_from(0b01_00_01_11));
    assert_eq!(Ok(opcodes::JMP), Opcode::try_from(0b01_00_01_00));
    assert_eq!(Ok(opcodes::CALL), Opcode::try_from(0b01_00_01_01));
    assert_eq!(Ok(opcodes::JMPR), Opcode::try_from(0b01_01_11_11));
    assert_eq!(Ok(opcodes::CALLR), Opcode::try_from(0b01_01_11_00));
    assert_eq!(Ok(opcodes::SYSCALL), Opcode::try_from(0b01_01_11_01));
    assert_eq!(Ok(opcodes::BREAK), Opcode::try_from(0b01_01_00_11));

    assert!(Opcode::try_from(0b00_00_00_11).is_err());
    assert!(Opcode::try_from(0b01_01_00_00).is_err());
}
