use constants::*;
use error::Result;
use tryte;
use tryte::Tryte;
use ternary::Ternary;
use registers::RegisterFile;
use opcodes;
use opcodes::Opcode;

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
        self.load_next_instruction();
        match self.current_opcode()? {
            opcodes::AND => self.op_and(),
            opcodes::OR => self.op_or(),
            opcodes::TMUL => self.op_tmul(),
            opcodes::TCMP => self.op_tcmp(),
            opcodes::CMP => self.op_cmp(),
            opcodes::SHF => self.op_shf(),
            opcodes::ADD => self.op_add(),
            opcodes::MUL => self.op_mul(),
            opcodes::DIV => self.op_div(),
            opcodes::ANDI => self.op_andi(),
            opcodes::ORI => self.op_ori(),
            opcodes::TMULI => self.op_tmuli(),
            opcodes::TCMPI => self.op_tcmpi(),
            opcodes::SHFI => self.op_shfi(),
            opcodes::ADDI => self.op_addi(),
            opcodes::LUI => self.op_lui(),
            opcodes::LSR => self.op_lsr(),
            opcodes::SSR => self.op_ssr(),
            opcodes::LT => self.op_lt(),
            opcodes::LH => self.op_lh(),
            opcodes::LW => self.op_lw(),
            opcodes::ST => self.op_st(),
            opcodes::SH => self.op_sh(),
            opcodes::SW => self.op_sw(),
            opcodes::BT => self.op_bt(),
            opcodes::B0 => self.op_b0(),
            opcodes::B1 => self.op_b1(),
            opcodes::BT0 => self.op_bt0(),
            opcodes::BT1 => self.op_bt1(),
            opcodes::B01 => self.op_b01(),
            opcodes::JMP => self.op_jmp(),
            opcodes::CALL => self.op_call(),
            opcodes::JMPR => self.op_jmpr(),
            opcodes::CALLR => self.op_callr(),
            opcodes::SYSCALL => self.op_syscall(),
            opcodes::BREAK => self.op_break(),
            _ => unreachable!(),
        }
    }

    fn load_next_instruction(&mut self) {
        let i = self.pc as usize;
        let j = i + WORD_LEN;
        self.pc += WORD_LEN;

        let src = &self.memory[i..j];
        let mut dest = &mut self.scratch_space[0..WORD_LEN];
        dest.copy_from_slice(src);
    }

    fn current_opcode(&self) -> Result<Opcode> {
        let instruction = &self.scratch_space[0..WORD_LEN];
        let opcode_trit4 = instruction.get_tryte(0).low_trit4();
        Opcode::from_trit4(opcode_trit4)
    }

    fn op_and(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_or(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_tmul(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_tcmp(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_cmp(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_shf(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_add(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_mul(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_div(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_andi(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_ori(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_tmuli(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_tcmpi(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_shfi(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_addi(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_lui(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_lsr(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_ssr(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_lt(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_lh(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_lw(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_st(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_sh(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_sw(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_bt(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_b0(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_b1(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_bt0(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_bt1(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_b01(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_jmp(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_call(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_jmpr(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_callr(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_syscall(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn op_break(&mut self) -> Result<()> {
        unimplemented!()
    }
}
