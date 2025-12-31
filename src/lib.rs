#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_docs_in_private_items,
    clippy::upper_case_acronyms
)]
#![allow(unused)] // necessary until there are binaries

mod error;
mod instructions;
mod opcodes;
mod operands;
mod registers;
mod vm;
