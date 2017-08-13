#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy",
           allow(cast_possible_truncation, cast_possible_wrap, cast_sign_loss,
                   missing_docs_in_private_items, pub_enum_variant_names))]

// necessary until there are binaries
#![allow(unused)]

#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]

extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate phf;

mod ternary;
mod error;
mod registers;
mod opcodes;
mod operands;
mod instructions;
mod vm;
