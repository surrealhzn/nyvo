use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    AesGcmSiv(aes_gcm_siv::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::AesGcmSiv(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<aes_gcm_siv::Error> for Error {
    fn from(err: aes_gcm_siv::Error) -> Self {
        Error::AesGcmSiv(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
