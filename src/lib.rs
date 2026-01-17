#![deny(clippy::all, clippy::pedantic)]
#![allow(unused)] // necessary until there are binaries

mod error;
mod instructions;
mod opcodes;
mod operands;
mod registers;
mod vm;
