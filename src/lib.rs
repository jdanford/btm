#![feature(try_from)]
#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate lazy_static;
extern crate phf;

mod trit;
mod tryte;
#[cfg(test)]
mod tests;
