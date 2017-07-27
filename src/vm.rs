use error::Result;
use ternary;
use ternary::constants::*;
use ternary::tryte;
use ternary::Tryte;
use ternary::Ternary;
use registers;
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
            Instruction::And(ref operands) => self.op_and(operands),
            Instruction::Or(ref operands) => self.op_or(operands),
            Instruction::Tmul(ref operands) => self.op_tmul(operands),
            Instruction::Tcmp(ref operands) => self.op_tcmp(operands),
            Instruction::Cmp(ref operands) => self.op_cmp(operands),
            Instruction::Shf(ref operands) => self.op_shf(operands),
            Instruction::Add(ref operands) => self.op_add(operands),
            Instruction::Mul(ref operands) => self.op_mul(operands),
            Instruction::Div(ref operands) => self.op_div(operands),
            Instruction::Andi(ref operands) => self.op_andi(operands),
            Instruction::Ori(ref operands) => self.op_ori(operands),
            Instruction::Tmuli(ref operands) => self.op_tmuli(operands),
            Instruction::Tcmpi(ref operands) => self.op_tcmpi(operands),
            Instruction::Shfi(ref operands) => self.op_shfi(operands),
            Instruction::Addi(ref operands) => self.op_addi(operands),
            Instruction::Lui(ref operands) => self.op_lui(operands),
            Instruction::Lsr(ref operands) => self.op_lsr(operands),
            Instruction::Ssr(ref operands) => self.op_ssr(operands),
            Instruction::Lt(ref operands) => self.op_lt(operands),
            Instruction::Lh(ref operands) => self.op_lh(operands),
            Instruction::Lw(ref operands) => self.op_lw(operands),
            Instruction::St(ref operands) => self.op_st(operands),
            Instruction::Sh(ref operands) => self.op_sh(operands),
            Instruction::Sw(ref operands) => self.op_sw(operands),
            Instruction::BT(ref operands) => self.op_bt(operands),
            Instruction::B0(ref operands) => self.op_b0(operands),
            Instruction::B1(ref operands) => self.op_b1(operands),
            Instruction::BT0(ref operands) => self.op_bt0(operands),
            Instruction::BT1(ref operands) => self.op_bt1(operands),
            Instruction::B01(ref operands) => self.op_b01(operands),
            Instruction::Jmp(ref operands) => self.op_jmp(operands),
            Instruction::Call(ref operands) => self.op_call(operands),
            Instruction::Jmpr(ref operands) => self.op_jmpr(operands),
            Instruction::Callr(ref operands) => self.op_callr(operands),
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

    fn do_simple_rrr<F>(&mut self, operands: &operands::RRR, f: F) -> Result<()>
    where
        F: Fn(&mut [Tryte], &[Tryte], &[Tryte]),
    {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];

        {
            let lhs = &self.registers[operands.lhs];
            let rhs = &self.registers[operands.rhs];
            f(tmp_dest, lhs, rhs);
        }

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn do_simple_rri<F>(&mut self, operands: &operands::RRI, f: F) -> Result<()>
    where
        F: Fn(&mut [Tryte], &[Tryte], &[Tryte]),
    {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];

        {
            let lhs = &self.registers[operands.src];
            let rhs = &operands.immediate;
            f(tmp_dest, lhs, rhs);
        }

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_and(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::and)
    }

    fn op_or(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::or)
    }

    fn op_tmul(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::tmul)
    }

    fn op_tcmp(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::tcmp)
    }

    fn op_cmp(&mut self, operands: &operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_shf(&mut self, operands: &operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_add(&mut self, operands: &operands::RRR) -> Result<()> {
        unimplemented!()
    }

    fn op_mul(&mut self, operands: &operands::RR) -> Result<()> {
        unimplemented!()
    }

    fn op_div(&mut self, operands: &operands::RR) -> Result<()> {
        unimplemented!()
    }

    fn op_andi(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::and)
    }

    fn op_ori(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::or)
    }

    fn op_tmuli(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::tmul)
    }

    fn op_tcmpi(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::tcmp)
    }

    fn op_shfi(&mut self, operands: &operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_addi(&mut self, operands: &operands::RRI) -> Result<()> {
        unimplemented!()
    }

    fn op_lui(&mut self, operands: &operands::RI) -> Result<()> {
        unimplemented!()
    }

    fn op_lsr(&mut self, operands: &operands::LoadSystem) -> Result<()> {
        unimplemented!()
    }

    fn op_ssr(&mut self, operands: &operands::StoreSystem) -> Result<()> {
        unimplemented!()
    }

    fn op_lt(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_lh(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_lw(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_st(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_sh(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_sw(&mut self, operands: &operands::Memory) -> Result<()> {
        unimplemented!()
    }

    fn op_bt(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b0(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b1(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_bt0(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_bt1(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_b01(&mut self, operands: &operands::Branch) -> Result<()> {
        unimplemented!()
    }

    fn op_jmp(&mut self, operands: &operands::Jump) -> Result<()> {
        unimplemented!()
    }

    fn op_call(&mut self, operands: &operands::Jump) -> Result<()> {
        unimplemented!()
    }

    fn op_jmpr(&mut self, operands: &operands::R) -> Result<()> {
        unimplemented!()
    }

    fn op_callr(&mut self, operands: &operands::R) -> Result<()> {
        unimplemented!()
    }

    fn op_syscall(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_break(&mut self) -> Result<()> {
        unimplemented!()
    }
}
