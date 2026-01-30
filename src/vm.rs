use std::ops::{BitAnd, BitOr, Mul, Range, RangeInclusive};

use ternary::trit::{_0, _1, _T};
use ternary::{T12, T24, T48, TInt, Trit, Tryte, tables::TRIT4_TO_I8, trit, tryte};

use crate::error::{Error, Result};
use crate::inst::Inst;
use crate::operands;
use crate::registers::{self, Register, Registers};

const TRIT3_POS_OFFSET: i8 = 13;

pub struct VM {
    running: bool,
    pc: i32,
    registers: Registers,
    memory: Vec<Tryte>,
}

impl VM {
    pub fn new(memory_size: u32) -> Self {
        let memory = vec![Tryte::ZERO; memory_size as usize];

        VM {
            running: false,
            pc: 0,
            registers: Registers::new(),
            memory,
        }
    }

    pub fn run(&mut self, pc: i32) -> Result<()> {
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
            Inst::And(operands) => self.op_and(operands),
            Inst::Or(operands) => self.op_or(operands),
            Inst::Tmul(operands) => self.op_tmul(operands),
            Inst::Tcmp(operands) => self.op_tcmp(operands),
            Inst::Cmp(operands) => self.op_cmp(operands),
            Inst::Shf(operands) => self.op_shf(operands),
            Inst::Add(operands) => self.op_add(operands),
            Inst::Mul(operands) => self.op_mul(operands),
            Inst::Div(operands) => self.op_div(operands),
            Inst::Andi(operands) => self.op_andi(operands),
            Inst::Ori(operands) => self.op_ori(operands),
            Inst::Tmuli(operands) => self.op_tmuli(operands),
            Inst::Tcmpi(operands) => self.op_tcmpi(operands),
            Inst::Shfi(operands) => self.op_shfi(operands),
            Inst::Addi(operands) => self.op_addi(operands),
            Inst::Lui(operands) => self.op_lui(operands),
            Inst::Lt(operands) => self.op_lt(operands),
            Inst::Lh(operands) => self.op_lh(operands),
            Inst::Lw(operands) => self.op_lw(operands),
            Inst::St(operands) => self.op_st(operands),
            Inst::Sh(operands) => self.op_sh(operands),
            Inst::Sw(operands) => self.op_sw(operands),
            Inst::BT(operands) => self.op_bt(operands),
            Inst::B0(operands) => self.op_b0(operands),
            Inst::B1(operands) => self.op_b1(operands),
            Inst::BT0(operands) => self.op_bt0(operands),
            Inst::BT1(operands) => self.op_bt1(operands),
            Inst::B01(operands) => self.op_b01(operands),
            Inst::Bal(operands) => self.op_bal(operands),
            Inst::J(operands) => self.op_j(operands),
            Inst::Jal(operands) => self.op_jal(operands),
            Inst::Jr(operands) => self.op_jr(operands),
            Inst::Jalr(operands) => self.op_jalr(operands),
            Inst::Syscall(_) => self.op_syscall(),
            Inst::Break(_) => self.op_break(),
        }

        Ok(())
    }

    fn next_instruction(&mut self) -> Result<Inst> {
        let range = self.memory_range(self.pc, 4, 4)?;
        self.pc += 4;

        let trytes = &self.memory[range];
        let word = T24::try_from(trytes).unwrap();
        Inst::from_word(word)
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

        self.registers[registers::LO] = quotient;
        self.registers[registers::HI] = remainder;
    }

    fn op_andi(&mut self, operands: operands::RRI) {
        self.simple_rri(operands, |r, i| r & i.resize());
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

    fn op_lt(&mut self, operands: operands::RRO) {
        self.load::<1>(operands);
    }

    fn op_lh(&mut self, operands: operands::RRO) {
        self.load::<2>(operands);
    }

    fn op_lw(&mut self, operands: operands::RRO) {
        self.load::<4>(operands);
    }

    fn op_st(&mut self, operands: operands::RRO) {
        self.store::<1>(operands);
    }

    fn op_sh(&mut self, operands: operands::RRO) {
        self.store::<2>(operands);
    }

    fn op_sw(&mut self, operands: operands::RRO) {
        self.store::<4>(operands);
    }

    fn op_bt(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, 0, 0);
    }

    fn op_b0(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, offset, 0);
    }

    fn op_b1(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, 0, offset);
    }

    fn op_bt0(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, offset, 0);
    }

    fn op_bt1(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, offset, 0, offset);
    }

    fn op_b01(&mut self, operands: operands::RO) {
        let selector = self.branch_selector(operands);
        let offset = operands.offset.try_into_int().unwrap();
        self.branch(selector, 0, offset, offset);
    }

    fn op_bal(&mut self, operands: operands::O) {
        let offset: i32 = operands.offset.try_into_int().unwrap();
        self.save_pc();
        self.pc += offset;
    }

    fn op_j(&mut self, operands: operands::A) {
        self.pc = operands.addr.try_into_int().unwrap();
    }

    fn op_jal(&mut self, operands: operands::A) {
        self.save_pc();
        self.pc = operands.addr.try_into_int().unwrap();
    }

    fn op_jr(&mut self, operands: operands::R) {
        self.pc = self.registers[operands.src].try_into_int().unwrap();
    }

    fn op_jalr(&mut self, operands: operands::R) {
        self.save_pc();
        self.pc = self.registers[operands.src].try_into_int().unwrap();
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

    #[allow(clippy::cast_sign_loss)]
    fn branch_selector(&self, operands: operands::RO) -> Trit {
        let src = self.registers[operands.src];
        src.trit(0)
    }

    fn branch(&mut self, selector: Trit, offset_t: i32, offset_0: i32, offset_1: i32) {
        let mut jump_table = [0; 4];
        jump_table[_T.into_index()] = offset_t;
        jump_table[_0.into_index()] = offset_0;
        jump_table[_1.into_index()] = offset_1;

        let i = selector.into_index();
        let offset = jump_table[i];
        self.pc += offset;
    }

    fn load<const N: usize>(&mut self, operands: operands::RRO) -> Result<()> {
        let addr = self.memory_op_addr(operands);
        let range = self.memory_range(addr, N, N)?;
        let trytes = &self.memory[range];
        let src = TInt::<N>::try_from(trytes).unwrap();

        self.registers[operands.dest] = src.resize();
        self.registers[registers::ZERO] = T24::ZERO;
        Ok(())
    }

    fn store<const N: usize>(&mut self, operands: operands::RRO) -> Result<()> {
        let addr = self.memory_op_addr(operands);
        let range = self.memory_range(addr, N, N)?;
        let trytes = &mut self.memory[range];
        let dest: &mut [Tryte; N] = trytes.try_into().unwrap();

        let src = self.registers[operands.src];
        *dest = src.resize().into_trytes();
        Ok(())
    }

    fn memory_op_addr(&mut self, operands: operands::RRO) -> i32 {
        let base_addr: i32 = self.registers[operands.dest].try_into_int().unwrap();
        let offset: i32 = operands.offset.try_into_int().unwrap();
        base_addr + offset
    }

    pub fn memory_range(&self, addr: i32, size: usize, align: usize) -> Result<Range<usize>> {
        let size_i32 = i32::try_from(size).unwrap();
        let align_i32 = i32::try_from(align).unwrap();

        if addr % align_i32 != 0 {
            return Err(Error::InvalidAlignment(addr, align));
        }

        let addr_bounds = self.addr_bounds();
        let addr_end = addr + size_i32;
        if !addr_bounds.contains(&addr) || !addr_bounds.contains(&(addr_end - 1)) {
            return Err(Error::InvalidAddress(addr));
        }

        let index_start = usize::try_from(addr - addr_bounds.start).unwrap();
        let index_end = index_start + size;
        Ok(index_start..index_end)
    }

    fn addr_bounds(&self) -> Range<i32> {
        let memory_size = self.memory.len();
        let offset = i32::try_from(memory_size / 2).unwrap();
        (-offset)..offset
    }

    fn save_pc(&mut self) {
        let value = T24::try_from_int(self.pc).unwrap();
        self.registers[registers::RA] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_range_tryte() {
        let vm = VM::new(2);
        assert_eq!(0..1, vm.memory_range(-1, 1, 1).unwrap());
        assert_eq!(1..2, vm.memory_range(0, 1, 1).unwrap());

        assert!(vm.memory_range(-2, 1, 1).is_err());
        assert!(vm.memory_range(1, 1, 1).is_err());
    }

    #[test]
    fn memory_range_half() {
        let vm = VM::new(4);
        assert_eq!(0..2, vm.memory_range(-2, 2, 2).unwrap());
        assert_eq!(2..4, vm.memory_range(0, 2, 2).unwrap());

        assert!(vm.memory_range(-3, 4, 4).is_err());
        assert!(vm.memory_range(-1, 4, 4).is_err());
        assert!(vm.memory_range(1, 4, 4).is_err());
        assert!(vm.memory_range(2, 4, 4).is_err());
    }

    #[test]
    fn memory_range_word() {
        let vm = VM::new(8);
        assert_eq!(0..4, vm.memory_range(-4, 4, 4).unwrap());
        assert_eq!(4..8, vm.memory_range(0, 4, 4).unwrap());

        assert!(vm.memory_range(-5, 4, 4).is_err());
        assert!(vm.memory_range(-3, 4, 4).is_err());
        assert!(vm.memory_range(-2, 4, 4).is_err());
        assert!(vm.memory_range(-1, 4, 4).is_err());
        assert!(vm.memory_range(1, 4, 4).is_err());
        assert!(vm.memory_range(2, 4, 4).is_err());
        assert!(vm.memory_range(3, 4, 4).is_err());
        assert!(vm.memory_range(4, 4, 4).is_err());
    }
}
