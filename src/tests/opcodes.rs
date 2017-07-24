use opcodes;
use opcodes::Opcode;

#[test]
fn opcode_from_trit4() {
    assert_eq!(Ok(opcodes::AND), Opcode::from_trit4(0b00_00_00_00));
    assert_eq!(Ok(opcodes::OR), Opcode::from_trit4(0b00_00_00_01));
    assert_eq!(Ok(opcodes::TMUL), Opcode::from_trit4(0b00_00_01_11));
    assert_eq!(Ok(opcodes::TCMP), Opcode::from_trit4(0b00_00_01_00));
    assert_eq!(Ok(opcodes::CMP), Opcode::from_trit4(0b00_00_01_01));
    assert_eq!(Ok(opcodes::SHF), Opcode::from_trit4(0b00_01_11_11));
    assert_eq!(Ok(opcodes::ADD), Opcode::from_trit4(0b00_01_11_00));
    assert_eq!(Ok(opcodes::MUL), Opcode::from_trit4(0b00_01_11_01));
    assert_eq!(Ok(opcodes::DIV), Opcode::from_trit4(0b00_01_00_11));
    assert_eq!(Ok(opcodes::ANDI), Opcode::from_trit4(0b00_01_00_00));
    assert_eq!(Ok(opcodes::ORI), Opcode::from_trit4(0b00_01_00_01));
    assert_eq!(Ok(opcodes::TMULI), Opcode::from_trit4(0b00_01_01_11));
    assert_eq!(Ok(opcodes::TCMPI), Opcode::from_trit4(0b00_01_01_00));
    assert_eq!(Ok(opcodes::SHFI), Opcode::from_trit4(0b00_01_01_01));
    assert_eq!(Ok(opcodes::ADDI), Opcode::from_trit4(0b01_11_11_11));
    assert_eq!(Ok(opcodes::LUI), Opcode::from_trit4(0b01_11_11_00));
    assert_eq!(Ok(opcodes::LSR), Opcode::from_trit4(0b01_11_11_01));
    assert_eq!(Ok(opcodes::SSR), Opcode::from_trit4(0b01_11_00_11));
    assert_eq!(Ok(opcodes::LT), Opcode::from_trit4(0b01_11_00_00));
    assert_eq!(Ok(opcodes::LH), Opcode::from_trit4(0b01_11_00_01));
    assert_eq!(Ok(opcodes::LW), Opcode::from_trit4(0b01_11_01_11));
    assert_eq!(Ok(opcodes::ST), Opcode::from_trit4(0b01_11_01_00));
    assert_eq!(Ok(opcodes::SH), Opcode::from_trit4(0b01_11_01_01));
    assert_eq!(Ok(opcodes::SW), Opcode::from_trit4(0b01_00_11_11));
    assert_eq!(Ok(opcodes::BT), Opcode::from_trit4(0b01_00_11_00));
    assert_eq!(Ok(opcodes::B0), Opcode::from_trit4(0b01_00_11_01));
    assert_eq!(Ok(opcodes::B1), Opcode::from_trit4(0b01_00_00_11));
    assert_eq!(Ok(opcodes::BT0), Opcode::from_trit4(0b01_00_00_00));
    assert_eq!(Ok(opcodes::BT1), Opcode::from_trit4(0b01_00_00_01));
    assert_eq!(Ok(opcodes::B01), Opcode::from_trit4(0b01_00_01_11));
    assert_eq!(Ok(opcodes::JMP), Opcode::from_trit4(0b01_00_01_00));
    assert_eq!(Ok(opcodes::CALL), Opcode::from_trit4(0b01_00_01_01));
    assert_eq!(Ok(opcodes::JMPR), Opcode::from_trit4(0b01_01_11_11));
    assert_eq!(Ok(opcodes::CALLR), Opcode::from_trit4(0b01_01_11_00));
    assert_eq!(Ok(opcodes::SYSCALL), Opcode::from_trit4(0b01_01_11_01));
    assert_eq!(Ok(opcodes::BREAK), Opcode::from_trit4(0b01_01_00_11));

    assert!(Opcode::from_trit4(0b00_00_00_11).is_err());
    assert!(Opcode::from_trit4(0b01_01_00_00).is_err());
}
