use std::result;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidOpcode(u8),
    InvalidRegister(u8),
    Ternary(ternary::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<ternary::Error> for Error {
    fn from(error: ternary::Error) -> Self {
        Error::Ternary(error)
    }
}
