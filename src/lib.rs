#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        clippy::missing_docs_in_private_items,
        clippy::non_ascii_literal,
        clippy::pub_enum_variant_names
    )
)]
#![allow(unused)] // necessary until there are binaries

extern crate byteorder;
extern crate lazy_static;
extern crate phf;

mod error;
mod instructions;
mod opcodes;
mod operands;
mod registers;
mod ternary;
mod vm;
