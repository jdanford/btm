use constants::*;
use error::Result;
use tryte;
use tryte::Tryte;
use registers::RegisterFile;

const SCRATCH_SPACE_LEN: usize = WORD_TRYTE_LEN * 4;

pub struct VM<'a> {
    running: bool,
    pc: u32,
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

    pub fn run(&mut self, pc: u32) -> Result<()> {
        self.pc = pc;
        self.running = true;

        while self.running {
            self.step()?;
        }

        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        Ok(())
    }
}
