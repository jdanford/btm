use std::result;

use ternary;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidEncoding(String),
    InvalidOpcode(u8),
    InvalidRegister(u8),
    TernaryError(ternary::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<ternary::Error> for Error {
    fn from(error: ternary::Error) -> Error {
        Error::TernaryError(error)
    }
}
