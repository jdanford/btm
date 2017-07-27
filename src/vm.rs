use error::Result;
use ternary::constants::*;
use ternary::tryte;
use ternary::Tryte;
use registers::RegisterFile;
use operands;
use instructions::Instruction;

const SCRATCH_SPACE_LEN: usize = WORD_LEN * 4;

pub struct VM<'a> {
    running: bool,
    pc: usize,
    registers: RegisterFile,
    scratch_space: [Tryte; SCRATCH_SPACE_LEN],
    memory: &'a mut [Tryte],
}

impl<'a> VM<'a> {
    pub fn new(memory: &'a mut [Tryte]) -> VM<'a> {
        VM {
            running: false,
            pc: 0,
            registers: RegisterFile::new(),
            scratch_space: [tryte::ZERO; SCRATCH_SPACE_LEN],
            memory: memory,
        }
    }

    pub fn run(&mut self, pc: usize) -> Result<()> {
        self.pc = pc;
        self.running = true;

        while self.running {
            self.step()?;
        }

        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        let instruction = self.next_instruction()?;
        match instruction {
            Instruction::And(operands) => self.op_and(operands),
            Instruction::Or(operands) => self.op_or(operands),
            Instruction::Tmul(operands) => self.op_tmul(operands),
            Instruction::Tcmp(operands) => self.op_tcmp(operands),
            Instruction::Cmp(operands) => self.op_cmp(operands),
            Instruction::Shf(operands) => self.op_shf(operands),
            Instruction::Add(operands) => self.op_add(operands),
            Instruction::Mul(operands) => self.op_mul(operands),
            Instruction::Div(operands) => self.op_div(operands),
            Instruction::Andi(operands) => self.op_andi(operands),
            Instruction::Ori(operands) => self.op_ori(operands),
            Instruction::Tmuli(operands) => self.op_tmuli(operands),
            Instruction::Tcmpi(operands) => self.op_tcmpi(operands),
            Instruction::Shfi(operands) => self.op_shfi(operands),
            Instruction::Addi(operands) => self.op_addi(operands),
            Instruction::Lui(operands) => self.op_lui(operands),
            Instruction::Lsr(operands) => self.op_lsr(operands),
            Instruction::Ssr(operands) => self.op_ssr(operands),
            Instruction::Lt(operands) => self.op_lt(operands),
            Instruction::Lh(operands) => self.op_lh(operands),
            Instruction::Lw(operands) => self.op_lw(operands),
            Instruction::St(operands) => self.op_st(operands),
            Instruction::Sh(operands) => self.op_sh(operands),
            Instruction::Sw(operands) => self.op_sw(operands),
            Instruction::BT(operands) => self.op_bt(operands),
            Instruction::B0(operands) => self.op_b0(operands),
            Instruction::B1(operands) => self.op_b1(operands),
            Instruction::BT0(operands) => self.op_bt0(operands),
            Instruction::BT1(operands) => self.op_bt1(operands),
            Instruction::B01(operands) => self.op_b01(operands),
            Instruction::Jmp(operands) => self.op_jmp(operands),
            Instruction::Call(operands) => self.op_call(operands),
            Instruction::Jmpr(operands) => self.op_jmpr(operands),
            Instruction::Callr(operands) => self.op_callr(operands),
            Instruction::Syscall => self.op_syscall(),
            Instruction::Break => self.op_break(),
            _ => unreachable!(),
        }
    }

    fn next_instruction(&mut self) -> Result<Instruction> {
        let i = self.pc as usize;
        let j = i + WORD_LEN;
        self.pc += WORD_LEN;

        let word = &self.memory[i..j];
        Instruction::from_word(word)
    }

    fn op_and(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_or(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_tmul(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_tcmp(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_cmp(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_shf(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_add(&mut self, operands: operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_mul(&mut self, operands: operands::RR) -> Result<()> {
        unimplemented!()
    }

    fn op_div(&mut self, operands: operands::RR) -> Result<()> {
        unimplemented!()
    }

    fn op_andi(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_ori(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_tmuli(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_tcmpi(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_shfi(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_addi(&mut self, operands: operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_lui(&mut self, operands: operands::RI) -> Result<()> {
        unimplemented!()
    }

    fn op_lsr(&mut self, operands: operands::LoadSystem) -> Result<()> {
        unimplemented!()
    }

    fn op_ssr(&mut self, operands: operands::StoreSystem) -> Result<()> {
        unimplemented!()
    }

    fn op_lt(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_lh(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_lw(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_st(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_sh(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_sw(&mut self, operands: operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_bt(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b0(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b1(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_bt0(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_bt1(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b01(&mut self, operands: operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_jmp(&mut self, operands: operands::Jump) -> Result<()> {
        unimplemented!()
    }

    fn op_call(&mut self, operands: operands::Jump) -> Result<()> {
        unimplemented!()
    }

    fn op_jmpr(&mut self, operands: operands::R) -> Result<()> {
        unimplemented!()
    }

    fn op_callr(&mut self, operands: operands::R) -> Result<()> {
        unimplemented!()
    }

    fn op_syscall(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_break(&mut self) -> Result<()> {
        unimplemented!()
    }
}
