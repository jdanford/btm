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
#[cfg(test)]
mod tests;
