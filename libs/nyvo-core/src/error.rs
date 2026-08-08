use thiserror::Error;

#[derive(Error, Debug)]
pub enum Err {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid archive: {0}")]
    InvalidArchive(String),

    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Wrong order of execution.")]
    ExecOrder,

    #[error("Specified file, directory or block not found in archive: {0}")]
    NotFoundInArchive(String),

    #[error("No matching key found for decryption.")]
    NoKeyFound,

    #[error(
        "This is not meant to happen, got `None` where `Some(T)` should be present. Please report this immediately."
    )]
    Safe,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(feature = "zip")]
impl From<zip::result::ZipError> for Err {
    fn from(err: zip::result::ZipError) -> Self {
        use zip::result::ZipError::*;

        match err {
            Io(err) => Err::Io(err),
            InvalidArchive(err) => Err::InvalidArchive(err.to_string()),
            UnsupportedArchive(err) => Err::Unsupported(err.to_string()),
            FileNotFound => Err::NotFoundInArchive("File not found in ZIP archive.".to_string()),
            InvalidPassword => Err::NoKeyFound,
            _ => Err::Unknown("Unknown ZIP handler error".to_string()),
        }
    }
}

pub type Res<T> = Result<T, Err>;
