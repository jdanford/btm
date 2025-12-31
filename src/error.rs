#[derive(Debug)]
pub enum Error {
    InvalidOpcode(u8),
    InvalidRegister(u8),
    Ternary(ternary::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ternary::Error> for Error {
    fn from(error: ternary::Error) -> Self {
        Error::Ternary(error)
    }
}
