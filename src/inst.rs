use ternary::{T24, Tryte};

use crate::error::Result;
use crate::opcodes::{self, Opcode};
use crate::operands::{self, Operand};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Inst {
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
    Lt(operands::RRO),
    Lh(operands::RRO),
    Lw(operands::RRO),
    St(operands::RRO),
    Sh(operands::RRO),
    Sw(operands::RRO),
    BT(operands::RO),
    B0(operands::RO),
    B1(operands::RO),
    BT0(operands::RO),
    BT1(operands::RO),
    B01(operands::RO),
    Bal(operands::O),
    J(operands::A),
    Jal(operands::A),
    Jr(operands::R),
    Jalr(operands::R),
    Syscall(operands::Empty),
    Break(operands::Empty),
}

impl Inst {
    pub fn from_word(word: T24) -> Result<Self> {
        let word_trytes = word.into_trytes();
        let opcode_trit4 = word_trytes[0].low_trit4();
        let opcode = Opcode::from_trit4(opcode_trit4)?;
        match opcode {
            opcodes::AND => operands::RRR::from_word(word).map(Inst::And),
            opcodes::OR => operands::RRR::from_word(word).map(Inst::Or),
            opcodes::TMUL => operands::RRR::from_word(word).map(Inst::Tmul),
            opcodes::TCMP => operands::RRR::from_word(word).map(Inst::Tcmp),
            opcodes::CMP => operands::RRR::from_word(word).map(Inst::Cmp),
            opcodes::SHF => operands::RRR::from_word(word).map(Inst::Shf),
            opcodes::ADD => operands::RRR::from_word(word).map(Inst::Add),
            opcodes::MUL => operands::RR::from_word(word).map(Inst::Mul),
            opcodes::DIV => operands::RR::from_word(word).map(Inst::Div),
            opcodes::ANDI => operands::RRI::from_word(word).map(Inst::Andi),
            opcodes::ORI => operands::RRI::from_word(word).map(Inst::Ori),
            opcodes::TMULI => operands::RRI::from_word(word).map(Inst::Tmuli),
            opcodes::TCMPI => operands::RRI::from_word(word).map(Inst::Tcmpi),
            opcodes::SHFI => operands::RRI::from_word(word).map(Inst::Shfi),
            opcodes::ADDI => operands::RRI::from_word(word).map(Inst::Addi),
            opcodes::LUI => operands::RI::from_word(word).map(Inst::Lui),
            opcodes::LT => operands::RRO::from_word(word).map(Inst::Lt),
            opcodes::LH => operands::RRO::from_word(word).map(Inst::Lh),
            opcodes::LW => operands::RRO::from_word(word).map(Inst::Lw),
            opcodes::ST => operands::RRO::from_word(word).map(Inst::St),
            opcodes::SH => operands::RRO::from_word(word).map(Inst::Sh),
            opcodes::SW => operands::RRO::from_word(word).map(Inst::Sw),
            opcodes::BT => operands::RO::from_word(word).map(Inst::BT),
            opcodes::B0 => operands::RO::from_word(word).map(Inst::B0),
            opcodes::B1 => operands::RO::from_word(word).map(Inst::B1),
            opcodes::BT0 => operands::RO::from_word(word).map(Inst::BT0),
            opcodes::BT1 => operands::RO::from_word(word).map(Inst::BT1),
            opcodes::B01 => operands::RO::from_word(word).map(Inst::B01),
            opcodes::BAL => operands::O::from_word(word).map(Inst::Bal),
            opcodes::J => operands::A::from_word(word).map(Inst::J),
            opcodes::JAL => operands::A::from_word(word).map(Inst::Jal),
            opcodes::JR => operands::R::from_word(word).map(Inst::Jr),
            opcodes::JALR => operands::R::from_word(word).map(Inst::Jalr),
            opcodes::SYSCALL => operands::Empty::from_word(word).map(Inst::Syscall),
            opcodes::BREAK => operands::Empty::from_word(word).map(Inst::Break),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use crate::registers;
    use ternary::test_constants::{T24_4096, T24_1073741824, TRYTE_6, TRYTE_NEG278};
    use ternary::trit::{self, _1};
    use ternary::{T12, tryte};

    pub const T12_4096: T12 = T24_4096.resize();

    #[test]
    #[allow(clippy::too_many_lines)]
    // TODO: use macro
    fn instruction_from_word() {
        assert_eq!(
            Inst::And(operands::RRR {
                dest: registers::ZERO,
                lhs: registers::ZERO,
                rhs: registers::ZERO,
            }),
            inst(concat!("000000000000", "0000", "0000", "0000")).unwrap()
        );
        assert_eq!(
            Inst::And(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "0000")).unwrap()
        );
        assert_eq!(
            Inst::Or(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "0001")).unwrap()
        );
        assert_eq!(
            Inst::Tmul(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "001T")).unwrap()
        );
        assert_eq!(
            Inst::Tcmp(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "0010")).unwrap()
        );
        assert_eq!(
            Inst::Cmp(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "0011")).unwrap()
        );
        assert_eq!(
            Inst::Shf(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "01TT")).unwrap()
        );
        assert_eq!(
            Inst::Add(operands::RRR {
                dest: registers::T0,
                lhs: registers::T1,
                rhs: registers::T2,
            }),
            inst(concat!("00000000", "1T01", "1T00", "1T0T", "01T0")).unwrap()
        );
        assert_eq!(
            Inst::Mul(operands::RR {
                lhs: registers::T0,
                rhs: registers::T1,
            }),
            inst(concat!("000000000000", "1T00", "1T0T", "01T1")).unwrap()
        );
        assert_eq!(
            Inst::Div(operands::RR {
                lhs: registers::T0,
                rhs: registers::T1,
            }),
            inst(concat!("000000000000", "1T00", "1T0T", "010T")).unwrap()
        );
        assert_eq!(
            Inst::Andi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "0100")).unwrap()
        );
        assert_eq!(
            Inst::Ori(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "0101")).unwrap()
        );
        assert_eq!(
            Inst::Tmuli(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "011T")).unwrap()
        );
        assert_eq!(
            Inst::Tcmpi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "0110")).unwrap()
        );
        assert_eq!(
            Inst::Shfi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "0111")).unwrap()
        );
        assert_eq!(
            Inst::Addi(operands::RRI {
                dest: registers::T0,
                src: registers::T1,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1TTT")).unwrap()
        );
        assert_eq!(
            Inst::Lui(operands::RI {
                dest: registers::T0,
                immediate: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "0000", "1T0T", "1TT0")).unwrap()
        );
        assert_eq!(
            Inst::Lt(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1TT1")).unwrap()
        );
        assert_eq!(
            Inst::Lh(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1T0T")).unwrap()
        );
        assert_eq!(
            Inst::Lw(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1T00")).unwrap()
        );
        assert_eq!(
            Inst::St(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1T01")).unwrap()
        );
        assert_eq!(
            Inst::Sh(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1T1T")).unwrap()
        );
        assert_eq!(
            Inst::Sw(operands::RRO {
                dest: registers::T0,
                src: registers::T1,
                offset: T12_4096,
            }),
            inst(concat!("0001T0T0TT01", "1T00", "1T0T", "1T10")).unwrap()
        );
        assert_eq!(
            Inst::BT(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "1T11")).unwrap()
        );
        assert_eq!(
            Inst::B0(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "10TT")).unwrap()
        );
        assert_eq!(
            Inst::B1(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "10T0")).unwrap()
        );
        assert_eq!(
            Inst::BT0(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "10T1")).unwrap()
        );
        assert_eq!(
            Inst::BT1(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "100T")).unwrap()
        );
        assert_eq!(
            Inst::B01(operands::RO {
                src: registers::T0,
                offset: T24_4096,
            }),
            inst(concat!("00000001T0T0TT01", "1T0T", "1000")).unwrap()
        );
        assert_eq!(
            Inst::Bal(operands::O { offset: T24_4096 }),
            inst(concat!("00000001T0T0TT01", "0000", "1001")).unwrap()
        );
        assert_eq!(
            Inst::J(operands::A {
                addr: T24_1073741824,
            }),
            inst(concat!("10T10T11110T1T0T0T01", "101T")).unwrap()
        );
        assert_eq!(
            Inst::Jal(operands::A {
                addr: T24_1073741824,
            }),
            inst(concat!("10T10T11110T1T0T0T01", "1010")).unwrap()
        );
        assert_eq!(
            Inst::Jr(operands::R { src: registers::T0 }),
            inst(concat!("0000000000000000", "1T0T", "1011")).unwrap()
        );
        assert_eq!(
            Inst::Jalr(operands::R { src: registers::T0 }),
            inst(concat!("0000000000000000", "1T0T", "11TT")).unwrap()
        );
        assert_eq!(
            Inst::Syscall(operands::Empty),
            inst(concat!("00000000000000000000", "11T0")).unwrap()
        );
        assert_eq!(
            Inst::Break(operands::Empty),
            inst(concat!("00000000000000000000", "11T1")).unwrap()
        );

        assert!(inst(concat!("00000000000000000000", "000T")).is_err());
        assert!(inst(concat!("00000000", "1T00", "1T0T", "T100", "0000")).is_err());
        assert!(inst(concat!("00000000000000000000", "110T")).is_err());
    }

    fn inst(s: &str) -> Result<Inst> {
        let word = T24::from_trit_str(s)?;
        Inst::from_word(word)
    }
}
