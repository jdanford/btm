use std::ops::{BitAnd, BitOr, Mul};

use ternary::trit::{_0, _1, _T};
use ternary::{T12, T24, T48, TInt, Trit, Tryte, tables::TRIT4_TO_I8, trit, tryte};

use crate::error::Result;
use crate::instructions::Instruction;
use crate::operands;
use crate::registers::{self, Register, Registers};

const SCRATCH_SPACE_LEN: usize = 4;
const TRIT3_POS_OFFSET: i8 = 13;

pub struct VM<'a> {
    running: bool,
    pc: u32,
    registers: Registers,
    memory: &'a mut [Tryte],
}

impl<'a> VM<'a> {
    pub fn new(memory: &'a mut [Tryte]) -> Self {
        VM {
            running: false,
            pc: 0,
            registers: Registers::new(),
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
        self.pc += 4;

        let word = T24::try_from(&self.memory[i..][..4]).unwrap();
        Instruction::from_word(word)
    }

    fn op_and(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, T24::bitand);
    }

    fn op_or(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, T24::bitor);
    }

    fn op_tmul(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, T24::tmul);
    }

    fn op_tcmp(&mut self, operands: operands::RRR) {
        self.simple_rrr(operands, T24::tcmp);
    }

    fn op_cmp(&mut self, operands: operands::RRR) {
        let lhs = self.registers[operands.lhs];
        let rhs = self.registers[operands.rhs];
        let cmp_trit = lhs.cmp_trit(rhs);

        self.registers[operands.dest] = T24::ZERO;
        self.registers[operands.dest].set_trit(0, cmp_trit);
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn op_shf(&mut self, operands: operands::RRR) {
        let offset = self.registers[operands.rhs].try_into_int().unwrap();
        self.shift(operands.dest, operands.lhs, offset);
    }

    fn op_add(&mut self, operands: operands::RRR) {
        let lhs = self.registers[operands.lhs];
        let rhs = self.registers[operands.rhs];
        let (sum, carry) = lhs.add_with_carry(rhs, _0);

        self.registers[operands.dest] = sum;
        self.registers[registers::HI] = T24::ZERO;
        self.registers[registers::HI].set_trit(0, carry);
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn op_mul(&mut self, operands: operands::RR) {
        let lhs: T48 = self.registers[operands.lhs].resize();
        let rhs: T48 = self.registers[operands.rhs].resize();
        let product = lhs * rhs;

        let product_trytes = product.into_trytes();
        let lo = T24::try_from(&product_trytes[..4]).unwrap();
        let hi = T24::try_from(&product_trytes[4..]).unwrap();

        self.registers[registers::LO] = lo;
        self.registers[registers::HI] = hi;
    }

    fn op_div(&mut self, operands: operands::RR) {
        let lhs = self.registers[operands.lhs];
        let rhs = self.registers[operands.rhs];
        let (quotient, remainder) = lhs.div_rem(rhs);

        self.registers[registers::HI] = quotient;
        self.registers[registers::LO] = remainder;
    }

    fn op_andi(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, |r, i| r + i.resize());
    }

    fn op_ori(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, |r, i| r | i.resize());
    }

    fn op_tmuli(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, |r, i| r.tmul(i.resize()));
    }

    fn op_tcmpi(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, |r, i| r.tcmp(i.resize()));
    }

    fn op_shfi(&mut self, operands: operands::RRI) {
        let offset = operands.immediate.try_into_int().unwrap();
        self.shift(operands.dest, operands.src, offset);
    }

    fn op_addi(&mut self, operands: operands::RRI) {
        let lhs = self.registers[operands.src];
        let rhs = operands.immediate.resize();
        let (sum, carry) = lhs.add_with_carry(rhs, _0);

        self.registers[operands.dest] = sum;
        self.registers[registers::HI] = T24::ZERO;
        self.registers[registers::HI].set_trit(0, carry);
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn op_lui(&mut self, operands: operands::RI) {
        self.registers[operands.dest] = operands.immediate.resize() << 12;
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn op_lt(&mut self, operands: operands::Memory) {
        self.load::<1>(operands);
    }

    fn op_lh(&mut self, operands: operands::Memory) {
        self.load::<2>(operands);
    }

    fn op_lw(&mut self, operands: operands::Memory) {
        self.load::<4>(operands);
    }

    fn op_st(&mut self, operands: operands::Memory) {
        self.store::<1>(operands);
    }

    fn op_sh(&mut self, operands: operands::Memory) {
        self.store::<2>(operands);
    }

    fn op_sw(&mut self, operands: operands::Memory) {
        self.store::<4>(operands);
    }

    fn op_bt(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, 0, 0);
    }

    fn op_b0(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, offset, 0);
    }

    fn op_b1(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, 0, offset);
    }

    fn op_bt0(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, offset, 0);
    }

    fn op_bt1(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, 0, offset);
    }

    fn op_b01(&mut self, operands: operands::Branch) {
        let selector = self.get_branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, offset, offset);
    }

    fn op_jmp(&mut self, operands: operands::Jump) {
        let offset = operands.offset.try_into_int().unwrap();
        self.jump_relative(offset);
    }

    fn op_call(&mut self, operands: operands::Jump) {
        let offset = operands.offset.try_into_int().unwrap();
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
        F: Fn(T24, T24) -> T24,
    {
        let lhs = self.registers[operands.lhs];
        let rhs = self.registers[operands.rhs];
        let value = f(lhs, rhs);

        self.registers[operands.dest] = value;
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn simple_rri<F>(&mut self, operands: operands::RRI, f: F)
    where
        F: Fn(T24, T12) -> T24,
    {
        let lhs = self.registers[operands.src];
        let rhs = operands.immediate;
        let value = f(lhs, rhs);

        self.registers[operands.dest] = value;
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn shift(&mut self, dest_reg: Register, src_reg: Register, offset: isize) {
        let value = self.registers[src_reg];
        self.registers[dest_reg] = value.shf(offset);
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn get_branch_selector(&self, operands: operands::Branch) -> Trit {
        let src = self.registers[operands.src];
        let raw_index = TRIT4_TO_I8[operands.index as usize];
        #[allow(clippy::cast_sign_loss)]
        let i = (raw_index + TRIT3_POS_OFFSET) as usize;
        src.trit(i)
    }

    fn branch(&mut self, selector: Trit, offset_t: i32, offset_0: i32, offset_1: i32) {
        let mut jump_table = [0; 4];
        jump_table[_T.into_index()] = offset_t;
        jump_table[_0.into_index()] = offset_0;
        jump_table[_1.into_index()] = offset_1;

        let i = selector.into_index();
        let offset = jump_table[i];
        self.jump_relative(offset);
    }

    fn load<const N: usize>(&mut self, operands: operands::Memory) {
        let i = self.get_memory_addr(operands);
        let trytes = &self.memory[i..][..N];
        let src = TInt::<N>::try_from(trytes).unwrap();

        self.registers[operands.dest] = src.resize();
        self.registers[registers::ZERO] = T24::ZERO;
    }

    fn store<const N: usize>(&mut self, operands: operands::Memory) {
        let src = self.registers[operands.src];
        let i = self.get_memory_addr(operands);
        let trytes = &mut self.memory[i..][..N];

        let dest: &mut [Tryte; N] = trytes.try_into().unwrap();
        *dest = src.resize().into_trytes();
    }

    #[allow(clippy::cast_sign_loss)]
    fn get_memory_addr(&mut self, operands: operands::Memory) -> usize {
        let base_addr: i32 = self.registers[operands.dest].try_into_int().unwrap();
        let offset: i32 = operands.offset.try_into_int().unwrap();
        (base_addr + offset) as usize
    }

    fn get_r_offset(&self, operands: operands::R) -> i32 {
        let offset_src = &self.registers[operands.src];
        offset_src.try_into_int().unwrap()
    }

    fn save_pc(&mut self) {
        let value = T24::try_from_int(i64::from(self.pc)).unwrap();
        self.registers[registers::RA] = value;
    }

    fn jump_relative(&mut self, offset: i32) {
        self.pc = self.relative_pc(offset);
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn relative_pc(&self, offset: i32) -> u32 {
        (self.pc as i32 + offset) as u32
    }
}
