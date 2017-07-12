#![feature(associated_consts)]
#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

mod trit;
mod tryte;
#[cfg(test)]
mod tests;
