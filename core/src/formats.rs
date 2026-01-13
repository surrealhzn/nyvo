#[cfg(feature = "nyvo")]
pub mod nyvo;
pub mod unknown;
#[cfg(feature = "zip")]
pub mod zip;

pub enum ArchiveFormatType {
    #[cfg(feature = "nyvo")]
    Nyvo,
    #[cfg(feature = "zip")]
    Zip,

    Unknown,
}

pub trait ArchiveFormat {
    fn get_type(&self) -> ArchiveFormatType;
}
