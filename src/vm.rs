use error::Result;
use ternary;
use ternary::constants::*;
use ternary::tables::TRIT4_TO_I8;
use ternary::{Ternary, trit, Trit, tryte, Tryte};
use registers;
use registers::{StandardRegister, RegisterFile};
use operands;
use instructions::Instruction;

const SCRATCH_SPACE_LEN: usize = WORD_LEN * 4;

const TRIT3_POS_OFFSET: i8 = 13;

pub struct VM<'a> {
    running: bool,
    pc: u32,
    registers: RegisterFile,
    scratch_space: [Tryte; SCRATCH_SPACE_LEN],
    jump_table: [i32; 4],
    memory: &'a mut [Tryte],
}

impl<'a> VM<'a> {
    pub fn new(memory: &'a mut [Tryte]) -> VM<'a> {
        VM {
            running: false,
            pc: 0,
            registers: RegisterFile::new(),
            scratch_space: [tryte::ZERO; SCRATCH_SPACE_LEN],
            jump_table: [0; 4],
            memory: memory,
        }
    }

    pub fn run(&mut self, pc: u32) -> Result<()> {
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
        }
    }

    fn next_instruction(&mut self) -> Result<Instruction> {
        let i = self.pc as usize;
        let j = i + WORD_LEN;
        self.pc += WORD_LEN as u32;

        let word = &self.memory[i..j];
        Instruction::from_word(word)
    }

    fn op_and(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::and);
        Ok(())
    }

    fn op_or(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::or);
        Ok(())
    }

    fn op_tmul(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::tmul);
        Ok(())
    }

    fn op_tcmp(&mut self, operands: &operands::RRR) -> Result<()> {
        self.do_simple_rrr(operands, ternary::tcmp);
        Ok(())
    }

    fn op_cmp(&mut self, operands: &operands::RRR) -> Result<()> {
        let cmp_trit = {
            let lhs = &self.registers[operands.lhs];
            let rhs = &self.registers[operands.rhs];
            ternary::compare(lhs, rhs)
        };

        self.registers[operands.dest].clear();
        self.registers[registers::HI].set_trit(0, cmp_trit);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_shf(&mut self, operands: &operands::RRR) -> Result<()> {
        let offset = (&self.registers[operands.rhs]).into_i64() as isize;
        self.do_shift(operands.lhs, offset);
        self.copy_shift_result(operands.dest);
        Ok(())
    }

    fn op_add(&mut self, operands: &operands::RRR) -> Result<()> {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];
        let carry = {
            let lhs = &self.registers[operands.lhs];
            let rhs = &self.registers[operands.rhs];
            ternary::add(tmp_dest, lhs, rhs, trit::ZERO)
        };

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        self.registers[registers::HI].clear();
        self.registers[registers::HI].set_trit(0, carry);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_mul(&mut self, operands: &operands::RR) -> Result<()> {
        let i = WORD_LEN;
        let j = WORD_LEN * 2;

        let tmp_dest = &mut self.scratch_space[0..j];

        {
            let lhs = &self.registers[operands.lhs];
            let rhs = &self.registers[operands.rhs];
            ternary::multiply(tmp_dest, lhs, rhs)
        }

        self.registers[registers::LO].copy_from_slice(&tmp_dest[0..i]);
        self.registers[registers::HI].copy_from_slice(&tmp_dest[i..j]);
        Ok(())
    }

    fn op_div(&mut self, operands: &operands::RR) -> Result<()> {
        unimplemented!()
    }

    fn op_andi(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::and);
        Ok(())
    }

    fn op_ori(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::or);
        Ok(())
    }

    fn op_tmuli(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::tmul);
        Ok(())
    }

    fn op_tcmpi(&mut self, operands: &operands::RRI) -> Result<()> {
        self.do_simple_rri(operands, ternary::tcmp);
        Ok(())
    }

    fn op_shfi(&mut self, operands: &operands::RRI) -> Result<()> {
        let offset = operands.immediate.into_i64() as isize;
        self.do_shift(operands.src, offset);
        self.copy_shift_result(operands.dest);
        Ok(())
    }

    fn op_addi(&mut self, operands: &operands::RRI) -> Result<()> {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];
        let carry = {
            let lhs = &self.registers[operands.src];
            let rhs = &operands.immediate;
            ternary::add(tmp_dest, lhs, rhs, trit::ZERO)
        };

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        self.registers[registers::HI].clear();
        self.registers[registers::HI].set_trit(0, carry);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_lui(&mut self, operands: &operands::RI) -> Result<()> {
        {
            let i = WORD_LEN;
            let j = WORD_LEN * 2;
            let mut dest = &mut self.registers[operands.dest][i..j];
            dest.copy_from_slice(&operands.immediate);
        }

        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_lsr(&mut self, operands: &operands::LoadSystem) -> Result<()> {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];

        {
            let src = &self.registers[operands.src];
            tmp_dest.copy_from_slice(src);
        }

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        self.registers[registers::ZERO].clear();
        Ok(())
    }

    fn op_ssr(&mut self, operands: &operands::StoreSystem) -> Result<()> {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];

        {
            let src = &self.registers[operands.src];
            tmp_dest.copy_from_slice(src);
        }

        self.registers[operands.dest].copy_from_slice(tmp_dest);
        Ok(())
    }

    fn op_lt(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_load(operands, TRYTE_LEN);
        Ok(())
    }

    fn op_lh(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_load(operands, HALF_LEN);
        Ok(())
    }

    fn op_lw(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_load(operands, WORD_LEN);
        Ok(())
    }

    fn op_st(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_store(operands, TRYTE_LEN);
        Ok(())
    }

    fn op_sh(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_store(operands, HALF_LEN);
        Ok(())
    }

    fn op_sw(&mut self, operands: &operands::Memory) -> Result<()> {
        self.do_store(operands, WORD_LEN);
        Ok(())
    }

    fn op_bt(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, offset, 0, 0);
        Ok(())
    }

    fn op_b0(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, 0, offset, 0);
        Ok(())
    }

    fn op_b1(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, 0, 0, offset);
        Ok(())
    }

    fn op_bt0(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, offset, offset, 0);
        Ok(())
    }

    fn op_bt1(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, offset, 0, offset);
        Ok(())
    }

    fn op_b01(&mut self, operands: &operands::Branch) -> Result<()> {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.do_branch(selector, 0, offset, offset);
        Ok(())
    }

    fn op_jmp(&mut self, operands: &operands::Jump) -> Result<()> {
        let offset = self.get_jump_offset(operands);
        self.do_rel_jump(offset);
        Ok(())
    }

    fn op_call(&mut self, operands: &operands::Jump) -> Result<()> {
        let offset = self.get_jump_offset(operands);
        self.save_pc();
        self.do_rel_jump(offset);
        Ok(())
    }

    fn op_jmpr(&mut self, operands: &operands::R) -> Result<()> {
        let offset = self.get_r_offset(operands);
        self.do_rel_jump(offset);
        Ok(())
    }

    fn op_callr(&mut self, operands: &operands::R) -> Result<()> {
        let offset = self.get_r_offset(operands);
        self.save_pc();
        self.do_rel_jump(offset);
        Ok(())
    }

    fn op_syscall(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_break(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn do_simple_rrr<F>(&mut self, operands: &operands::RRR, f: F)
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
    }

    fn do_simple_rri<F>(&mut self, operands: &operands::RRI, f: F)
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
    }

    fn do_shift(&mut self, src_reg: StandardRegister, offset: isize) {
        let tmp_dest = &mut self.scratch_space[0..(WORD_LEN * 3)];
        let src = &self.registers[src_reg];
        ternary::shift(tmp_dest, src, offset);
    }

    fn copy_shift_result(&mut self, dest_reg: StandardRegister) {
        let i = WORD_LEN;
        let j = WORD_LEN * 2;
        let k = WORD_LEN * 3;

        {
            let src = &mut self.scratch_space[0..i];
            self.registers[registers::LO].copy_from_slice(src);
        }

        {
            let src = &mut self.scratch_space[i..j];
            self.registers[dest_reg].copy_from_slice(src);
        }

        {
            let src = &mut self.scratch_space[j..k];
            self.registers[registers::HI].copy_from_slice(src);
        }

        self.registers[registers::ZERO].clear();
    }

    fn get_branch_selector(&self, operands: &operands::Branch) -> Trit {
        let raw_index = TRIT4_TO_I8[operands.index as usize];
        let i = (raw_index + TRIT3_POS_OFFSET) as usize;
        let src = &self.registers[operands.src];
        src.get_trit(i)
    }

    fn do_branch(&mut self, selector: Trit, offset_t: i32, offset_0: i32, offset_1: i32) {
        self.jump_table[trit::NEG.into_index()] = offset_t;
        self.jump_table[trit::ZERO.into_index()] = offset_0;
        self.jump_table[trit::POS.into_index()] = offset_1;

        let i = selector.into_index();
        let offset = self.jump_table[i];
        self.do_rel_jump(offset);
    }

    fn do_load(&mut self, operands: &operands::Memory, len: usize) {
        let i = self.get_memory_addr(operands);
        let j = i + len;
        let src = &self.memory[i..j];

        {
            let dest_reg = &mut self.registers[operands.dest];
            dest_reg.clear();
            let mut dest = &mut dest_reg[..len];
            dest.copy_from_slice(src);
        }

        self.registers[registers::ZERO].clear();
    }

    fn do_store(&mut self, operands: &operands::Memory, len: usize) {
        let i = self.get_memory_addr(operands);
        let j = i + len;

        let dest = &mut self.memory[i..j];
        let src = &self.registers[operands.src][..len];
        dest.copy_from_slice(src);
    }

    fn get_memory_addr(&mut self, operands: &operands::Memory) -> usize {
        let base_addr = {
            let addr_src = &self.registers[operands.dest];
            addr_src.into_i64() as u32
        };
        let offset = self.get_memory_offset(operands);
        (base_addr as i32 + offset) as usize
    }

    fn get_jump_offset(&mut self, operands: &operands::Jump) -> i32 {
        let mut offset_dest = &mut self.scratch_space[0..WORD_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_branch_offset(&mut self, operands: &operands::Branch) -> i32 {
        let mut offset_dest = &mut self.scratch_space[0..HALF_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_memory_offset(&mut self, operands: &operands::Memory) -> i32 {
        let mut offset_dest = &mut self.scratch_space[0..HALF_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_r_offset(&self, operands: &operands::R) -> i32 {
        let offset_src = &self.registers[operands.src];
        offset_src.into_i64() as i32
    }

    fn save_pc(&mut self) {
        self.registers[registers::RA]
            .read_i64(self.pc as i64)
            .expect("ternary arithmetic error")
    }

    fn do_rel_jump(&mut self, offset: i32) {
        self.pc = self.rel_pc(offset);
    }

    fn rel_pc(&self, offset: i32) -> u32 {
        (self.pc as i32 + offset) as u32
    }
}
