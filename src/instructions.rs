use error::Result;
use tryte::Tryte;
use opcodes;
use opcodes::Opcode;
use operands;
use operands::Operand;

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
    pub fn from_word(word: &[Tryte]) -> Result<Instruction> {
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
