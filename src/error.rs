use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    IntegerOutOfBounds(i64, i64, i64),
    InvalidBitPattern(u64),
    InvalidCharacter(char),
    InvalidDataLength(usize, usize),
    InvalidIndex(usize),
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
            (&Error::InvalidString(ref s1), &Error::InvalidString(ref s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Eq for Error {}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}
