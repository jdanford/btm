use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    FormatError(fmt::Error),
    IntegerOutOfBounds(i64, i64, i64),
    InvalidBitPattern(u64),
    InvalidCharacter(char),
    InvalidDataLength(usize, usize),
    InvalidInstruction(String),
    InvalidOpcode(u8),
    InvalidRegister(u8),
    InvalidString(String),
    IoError(io::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::IntegerOutOfBounds(a1, b1, n1), &Error::IntegerOutOfBounds(a2, b2, n2)) => {
                a1 == a2 && b1 == b2 && n1 == n2
            }
            (&Error::InvalidBitPattern(n1), &Error::InvalidBitPattern(n2)) => n1 == n2,
            (&Error::InvalidCharacter(c1), &Error::InvalidCharacter(c2)) => c1 == c2,
            (&Error::InvalidDataLength(e1, a1), &Error::InvalidDataLength(e2, a2)) => {
                e1 == a1 && e2 == a2
            }
            (&Error::InvalidInstruction(ref s1), &Error::InvalidInstruction(ref s2)) |
            (&Error::InvalidString(ref s1), &Error::InvalidString(ref s2)) => s1 == s2,
            (&Error::InvalidOpcode(n1), &Error::InvalidOpcode(n2)) |
            (&Error::InvalidRegister(n1), &Error::InvalidRegister(n2)) => n1 == n2,
            _ => false,
        }
    }
}

impl Eq for Error {}

pub type Result<T> = result::Result<T, Error>;

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Error {
        Error::FormatError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}
