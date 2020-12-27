use std::result;

use crate::ternary;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidOpcode(u8),
    InvalidRegister(u8),
    TernaryError(ternary::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<ternary::Error> for Error {
    fn from(error: ternary::Error) -> Self {
        Error::TernaryError(error)
    }
}
