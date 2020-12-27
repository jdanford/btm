use crate::error::Result;
use crate::instructions::Instruction;
use crate::operands;
use crate::registers;
use crate::registers::{Register, RegisterFile, StandardRegister};
use crate::ternary;
use crate::ternary::constants::{HALF_LEN, TRYTE_LEN, WORD_LEN};
use crate::ternary::tables::TRIT4_TO_I8;
use crate::ternary::{trit, tryte, Ternary, Trit, Tryte};

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
    pub fn new(memory: &'a mut [Tryte]) -> Self {
        VM {
            running: false,
            pc: 0,
            registers: RegisterFile::new(),
            scratch_space: [tryte::ZERO; SCRATCH_SPACE_LEN],
            jump_table: [0; 4],
            memory,
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
        }

        Ok(())
    }

    fn next_instruction(&mut self) -> Result<Instruction> {
        let i = self.pc as usize;
        let j = i + WORD_LEN;
        self.pc += WORD_LEN as u32;

        let word = &self.memory[i..j];
        Instruction::from_word(word)
    }

    fn op_and(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, ternary::and);
    }

    fn op_or(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, ternary::or);
    }

    fn op_tmul(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, ternary::tmul);
    }

    fn op_tcmp(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, ternary::tcmp);
    }

    fn op_cmp(&mut self, operands: operands::RRR) {
        let cmp_trit = {
            let lhs = &self.registers[operands.lhs];
            let rhs = &self.registers[operands.rhs];
            ternary::compare(lhs, rhs)
        };

        self.registers[operands.dest].clear();
        self.registers[operands.dest].set_trit(0, cmp_trit);
        self.registers[registers::ZERO].clear();
    }

    fn op_shf(&mut self, operands: operands::RRR) {
        let offset = (&self.registers[operands.rhs]).into_i64() as isize;
        self.shift(operands.lhs, offset);
        self.copy_shift_result(operands.dest);
    }

    fn op_add(&mut self, operands: operands::RRR) {
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
    }

    fn op_mul(&mut self, operands: operands::RR) {
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
    }

    #[allow(clippy::unused_self)]
    fn op_div(&mut self, operands: operands::RR) {
        unimplemented!()
    }

    fn op_andi(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, ternary::and);
    }

    fn op_ori(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, ternary::or);
    }

    fn op_tmuli(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, ternary::tmul);
    }

    fn op_tcmpi(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, ternary::tcmp);
    }

    fn op_shfi(&mut self, operands: operands::RRI) {
        let offset = operands.immediate.into_i64() as isize;
        self.shift(operands.src, offset);
        self.copy_shift_result(operands.dest);
    }

    fn op_addi(&mut self, operands: operands::RRI) {
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
    }

    fn op_lui(&mut self, operands: operands::RI) {
        let i = HALF_LEN;
        let j = HALF_LEN * 2;

        {
            let dest = &mut self.registers[operands.dest];
            dest[0..i].clear();
            dest[i..j].copy_from_slice(&operands.immediate);
        }

        self.registers[registers::ZERO].clear();
    }

    fn op_lsr(&mut self, operands: operands::LoadSystem) {
        self.copy_register(operands.src, operands.dest);
        self.registers[registers::ZERO].clear();
    }

    fn op_ssr(&mut self, operands: operands::StoreSystem) {
        self.copy_register(operands.src, operands.dest);
    }

    fn op_lt(&mut self, operands: operands::Memory) {
        self.load(operands, TRYTE_LEN);
    }

    fn op_lh(&mut self, operands: operands::Memory) {
        self.load(operands, HALF_LEN);
    }

    fn op_lw(&mut self, operands: operands::Memory) {
        self.load(operands, WORD_LEN);
    }

    fn op_st(&mut self, operands: operands::Memory) {
        self.store(operands, TRYTE_LEN);
    }

    fn op_sh(&mut self, operands: operands::Memory) {
        self.store(operands, HALF_LEN);
    }

    fn op_sw(&mut self, operands: operands::Memory) {
        self.store(operands, WORD_LEN);
    }

    fn op_bt(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, offset, 0, 0);
    }

    fn op_b0(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, 0, offset, 0);
    }

    fn op_b1(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, 0, 0, offset);
    }

    fn op_bt0(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, offset, offset, 0);
    }

    fn op_bt1(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, offset, 0, offset);
    }

    fn op_b01(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = self.get_branch_offset(operands);
        self.branch(selector, 0, offset, offset);
    }

    fn op_jmp(&mut self, operands: operands::Jump) {
        let offset = self.get_jump_offset(operands);
        self.jump_relative(offset);
    }

    fn op_call(&mut self, operands: operands::Jump) {
        let offset = self.get_jump_offset(operands);
        self.save_pc();
        self.jump_relative(offset);
    }

    fn op_jmpr(&mut self, operands: operands::R) {
        let offset = self.get_r_offset(operands);
        self.jump_relative(offset);
    }

    fn op_callr(&mut self, operands: operands::R) {
        let offset = self.get_r_offset(operands);
        self.save_pc();
        self.jump_relative(offset);
    }

    #[allow(clippy::unused_self)]
    fn op_syscall(&mut self) {
        unimplemented!()
    }

    #[allow(clippy::unused_self)]
    fn op_break(&mut self) {
        unimplemented!()
    }

    fn simple_rrr<F>(&mut self, operands: operands::RRR, f: F)
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

    fn simple_rri<F>(&mut self, operands: operands::RRI, f: F)
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

    fn shift(&mut self, src_reg: StandardRegister, offset: isize) {
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

    fn copy_register<R: Register, S: Register>(&mut self, src_reg: R, dest_reg: S) {
        let tmp_dest = &mut self.scratch_space[0..WORD_LEN];

        {
            let src = &self.registers[src_reg];
            tmp_dest.copy_from_slice(src);
        }

        self.registers[dest_reg].copy_from_slice(tmp_dest);
    }

    fn get_branch_selector(&self, operands: operands::Branch) -> Trit {
        let raw_index = TRIT4_TO_I8[operands.index as usize];
        let i = (raw_index + TRIT3_POS_OFFSET) as usize;
        let src = &self.registers[operands.src];
        src.get_trit(i)
    }

    fn branch(&mut self, selector: Trit, offset_t: i32, offset_0: i32, offset_1: i32) {
        self.jump_table[trit::NEG.into_index()] = offset_t;
        self.jump_table[trit::ZERO.into_index()] = offset_0;
        self.jump_table[trit::POS.into_index()] = offset_1;

        let i = selector.into_index();
        let offset = self.jump_table[i];
        self.jump_relative(offset);
    }

    fn load(&mut self, operands: operands::Memory, len: usize) {
        let i = self.get_memory_addr(operands);
        let j = i + len;
        let src = &self.memory[i..j];

        {
            let dest_reg = &mut self.registers[operands.dest];
            dest_reg.clear();
            let dest = &mut dest_reg[..len];
            dest.copy_from_slice(src);
        }

        self.registers[registers::ZERO].clear();
    }

    fn store(&mut self, operands: operands::Memory, len: usize) {
        let i = self.get_memory_addr(operands);
        let j = i + len;

        let dest = &mut self.memory[i..j];
        let src = &self.registers[operands.src][..len];
        dest.copy_from_slice(src);
    }

    fn get_memory_addr(&mut self, operands: operands::Memory) -> usize {
        let base_addr = {
            let addr_src = &self.registers[operands.dest];
            addr_src.into_i64() as u32
        };
        let offset = self.get_memory_offset(operands);
        (base_addr as i32 + offset) as usize
    }

    fn get_jump_offset(&mut self, operands: operands::Jump) -> i32 {
        let offset_dest = &mut self.scratch_space[0..WORD_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_branch_offset(&mut self, operands: operands::Branch) -> i32 {
        let offset_dest = &mut self.scratch_space[0..HALF_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_memory_offset(&mut self, operands: operands::Memory) -> i32 {
        let offset_dest = &mut self.scratch_space[0..HALF_LEN];
        offset_dest.copy_from_slice(&operands.offset[..]);
        offset_dest.into_i64() as i32
    }

    fn get_r_offset(&self, operands: operands::R) -> i32 {
        let offset_src = &self.registers[operands.src];
        offset_src.into_i64() as i32
    }

    fn save_pc(&mut self) {
        self.registers[registers::RA]
            .read_i64(self.pc as i64)
            .expect("ternary arithmetic error")
    }

    fn jump_relative(&mut self, offset: i32) {
        self.pc = self.relative_pc(offset);
    }

    fn relative_pc(&self, offset: i32) -> u32 {
        (self.pc as i32 + offset) as u32
    }
}
