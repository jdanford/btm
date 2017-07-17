#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]

extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate phf;

mod error;
mod trit;
mod hyte;
mod tryte;
mod ternary;
#[cfg(test)]
mod tests;
