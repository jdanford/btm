#[derive(Debug)]
pub enum Error {
    InvalidOpcode(i8),
    InvalidRegister(i8),
    InvalidAddress(i64),
    InvalidAlignment(i64, usize),
    Ternary(ternary::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ternary::Error> for Error {
    fn from(error: ternary::Error) -> Self {
        Error::Ternary(error)
    }
}
