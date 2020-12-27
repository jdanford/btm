use crate::error::Result;
use crate::opcodes;
use crate::opcodes::Opcode;
use crate::operands;
use crate::operands::Operand;
use crate::ternary::Tryte;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    And(operands::RRR),
    Or(operands::RRR),
    Tmul(operands::RRR),
    Tcmp(operands::RRR),
    Cmp(operands::RRR),
    Shf(operands::RRR),
    Add(operands::RRR),
    Mul(operands::RR),
    Div(operands::RR),
    Andi(operands::RRI),
    Ori(operands::RRI),
    Tmuli(operands::RRI),
    Tcmpi(operands::RRI),
    Shfi(operands::RRI),
    Addi(operands::RRI),
    Lui(operands::RI),
    Lsr(operands::LoadSystem),
    Ssr(operands::StoreSystem),
    Lt(operands::Memory),
    Lh(operands::Memory),
    Lw(operands::Memory),
    St(operands::Memory),
    Sh(operands::Memory),
    Sw(operands::Memory),
    BT(operands::Branch),
    B0(operands::Branch),
    B1(operands::Branch),
    BT0(operands::Branch),
    BT1(operands::Branch),
    B01(operands::Branch),
    Jmp(operands::Jump),
    Call(operands::Jump),
    Jmpr(operands::R),
    Callr(operands::R),
    Syscall,
    Break,
}

impl Instruction {
    pub fn from_word(word: &[Tryte]) -> Result<Self> {
        let opcode_trit4 = word[0].low_trit4();
        let opcode = Opcode::from_trit4(opcode_trit4)?;
        match opcode {
            opcodes::AND => operands::RRR::from_word(word).map(Instruction::And),
            opcodes::OR => operands::RRR::from_word(word).map(Instruction::Or),
            opcodes::TMUL => operands::RRR::from_word(word).map(Instruction::Tmul),
            opcodes::TCMP => operands::RRR::from_word(word).map(Instruction::Tcmp),
            opcodes::CMP => operands::RRR::from_word(word).map(Instruction::Cmp),
            opcodes::SHF => operands::RRR::from_word(word).map(Instruction::Shf),
            opcodes::ADD => operands::RRR::from_word(word).map(Instruction::Add),
            opcodes::MUL => operands::RR::from_word(word).map(Instruction::Mul),
            opcodes::DIV => operands::RR::from_word(word).map(Instruction::Div),
            opcodes::ANDI => operands::RRI::from_word(word).map(Instruction::Andi),
            opcodes::ORI => operands::RRI::from_word(word).map(Instruction::Ori),
            opcodes::TMULI => operands::RRI::from_word(word).map(Instruction::Tmuli),
            opcodes::TCMPI => operands::RRI::from_word(word).map(Instruction::Tcmpi),
            opcodes::SHFI => operands::RRI::from_word(word).map(Instruction::Shfi),
            opcodes::ADDI => operands::RRI::from_word(word).map(Instruction::Addi),
            opcodes::LUI => operands::RI::from_word(word).map(Instruction::Lui),
            opcodes::LSR => operands::LoadSystem::from_word(word).map(Instruction::Lsr),
            opcodes::SSR => operands::StoreSystem::from_word(word).map(Instruction::Ssr),
            opcodes::LT => operands::Memory::from_word(word).map(Instruction::Lt),
            opcodes::LH => operands::Memory::from_word(word).map(Instruction::Lh),
            opcodes::LW => operands::Memory::from_word(word).map(Instruction::Lw),
            opcodes::ST => operands::Memory::from_word(word).map(Instruction::St),
            opcodes::SH => operands::Memory::from_word(word).map(Instruction::Sh),
            opcodes::SW => operands::Memory::from_word(word).map(Instruction::Sw),
            opcodes::BT => operands::Branch::from_word(word).map(Instruction::BT),
            opcodes::B0 => operands::Branch::from_word(word).map(Instruction::B0),
            opcodes::B1 => operands::Branch::from_word(word).map(Instruction::B1),
            opcodes::BT0 => operands::Branch::from_word(word).map(Instruction::BT0),
            opcodes::BT1 => operands::Branch::from_word(word).map(Instruction::BT1),
            opcodes::B01 => operands::Branch::from_word(word).map(Instruction::B01),
            opcodes::JMP => operands::Jump::from_word(word).map(Instruction::Jmp),
            opcodes::CALL => operands::Jump::from_word(word).map(Instruction::Call),
            opcodes::JMPR => operands::R::from_word(word).map(Instruction::Jmpr),
            opcodes::CALLR => operands::R::from_word(word).map(Instruction::Callr),
            opcodes::SYSCALL => operands::Empty::from_word(word).map(|_| Instruction::Syscall),
            opcodes::BREAK => operands::Empty::from_word(word).map(|_| Instruction::Break),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers;
    use crate::ternary::constants::WORD_LEN;
    use crate::ternary::test_constants::{TRYTE2_4096, TRYTE4_1073741824, TRYTE_6};
    use crate::ternary::trit;
    use crate::ternary::tryte;
    use crate::ternary::Ternary;

    #[test]
    #[allow(clippy::too_many_lines)]
    // TODO: use macro
    fn instruction_from_word() {
        assert_eq!(
            Instruction::And(operands::RRR {
                dest: registers::ZERO,
                lhs: registers::ZERO,
                rhs: registers::ZERO,
            }),
            instruction_from_trit_str(concat!("000000000000", "0000", "0000", "0000")).unwrap()
        );
        assert_eq!(
            Instruction::And(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "0000")).unwrap()
        );
        assert_eq!(
            Instruction::Or(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "0001")).unwrap()
        );
        assert_eq!(
            Instruction::Tmul(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "001T")).unwrap()
        );
        assert_eq!(
            Instruction::Tcmp(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "0010")).unwrap()
        );
        assert_eq!(
            Instruction::Cmp(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "0011")).unwrap()
        );
        assert_eq!(
            Instruction::Shf(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "01TT")).unwrap()
        );
        assert_eq!(
            Instruction::Add(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "1T00", "01T0")).unwrap()
        );
        assert_eq!(
            Instruction::Mul(operands::RR {
                lhs: registers::T0,
                rhs: registers::T1,
            }),
            instruction_from_trit_str(concat!("000000000000", "1T01", "1T00", "01T1")).unwrap()
        );
        assert_eq!(
            Instruction::Div(operands::RR {
                lhs: registers::T0,
                rhs: registers::T1,
            }),
            instruction_from_trit_str(concat!("000000000000", "1T01", "1T00", "010T")).unwrap()
        );
        assert_eq!(
            Instruction::Andi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "0100")).unwrap()
        );
        assert_eq!(
            Instruction::Ori(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "0101")).unwrap()
        );
        assert_eq!(
            Instruction::Tmuli(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "011T")).unwrap()
        );
        assert_eq!(
            Instruction::Tcmpi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "0110")).unwrap()
        );
        assert_eq!(
            Instruction::Shfi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "0111")).unwrap()
        );
        assert_eq!(
            Instruction::Addi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1TTT")).unwrap()
        );
        assert_eq!(
            Instruction::Lui(operands::RI {
                dest: registers::T0,
                immediate: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "0000", "1T00", "1TT0")).unwrap()
        );
        assert_eq!(
            Instruction::Lsr(operands::LoadSystem {
                dest: registers::T0,
                src: registers::ED,
            }),
            instruction_from_trit_str(concat!("000000000000", "0010", "1T00", "1TT1")).unwrap()
        );
        assert_eq!(
            Instruction::Ssr(operands::StoreSystem {
                dest: registers::ED,
                src: registers::T0,
            }),
            instruction_from_trit_str(concat!("000000000000", "1T00", "0010", "1T0T")).unwrap()
        );
        assert_eq!(
            Instruction::Lt(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1T00")).unwrap()
        );
        assert_eq!(
            Instruction::Lh(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1T01")).unwrap()
        );
        assert_eq!(
            Instruction::Lw(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1T1T")).unwrap()
        );
        assert_eq!(
            Instruction::St(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1T10")).unwrap()
        );
        assert_eq!(
            Instruction::Sh(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "1T11")).unwrap()
        );
        assert_eq!(
            Instruction::Sw(operands::Memory {
                dest: registers::T0,
                src: registers::T1,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1T01", "1T00", "10TT")).unwrap()
        );
        assert_eq!(
            Instruction::BT(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "10T0")).unwrap()
        );
        assert_eq!(
            Instruction::B0(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "10T1")).unwrap()
        );
        assert_eq!(
            Instruction::B1(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "100T")).unwrap()
        );
        assert_eq!(
            Instruction::BT0(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "1000")).unwrap()
        );
        assert_eq!(
            Instruction::BT1(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "1001")).unwrap()
        );
        assert_eq!(
            Instruction::B01(operands::Branch {
                src: registers::T0,
                index: TRYTE_6.low_trit4(),
                hint: trit::POS,
                offset: TRYTE2_4096,
            }),
            instruction_from_trit_str(concat!("0001T0T0TT01", "1", "1T0", "1T00", "101T")).unwrap()
        );
        assert_eq!(
            Instruction::Jmp(operands::Jump {
                offset: TRYTE4_1073741824,
            }),
            instruction_from_trit_str(concat!("10T10T11110T1T0T0T01", "1010")).unwrap()
        );
        assert_eq!(
            Instruction::Call(operands::Jump {
                offset: TRYTE4_1073741824,
            }),
            instruction_from_trit_str(concat!("10T10T11110T1T0T0T01", "1011")).unwrap()
        );
        assert_eq!(
            Instruction::Jmpr(operands::R { src: registers::T0 }),
            instruction_from_trit_str(concat!("0000000000000000", "1T00", "11TT")).unwrap()
        );
        assert_eq!(
            Instruction::Callr(operands::R { src: registers::T0 }),
            instruction_from_trit_str(concat!("0000000000000000", "1T00", "11T0")).unwrap()
        );
        assert_eq!(
            Instruction::Syscall,
            instruction_from_trit_str(concat!("00000000000000000000", "11T1")).unwrap()
        );
        assert_eq!(
            Instruction::Break,
            instruction_from_trit_str(concat!("00000000000000000000", "110T")).unwrap()
        );

        assert!(instruction_from_trit_str(concat!("00000000000000000000", "000T")).is_err());
        assert!(
            instruction_from_trit_str(concat!("00000000", "1T1T", "1T01", "T100", "0000")).is_err()
        );
    }

    fn instruction_from_trit_str(s: &str) -> Result<Instruction> {
        let mut word = [tryte::ZERO; WORD_LEN];
        word.read_trits(s)?;
        Instruction::from_word(&word)
    }
}
