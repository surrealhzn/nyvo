pub use super::{
    ArchiveFormat,
    ArchiveFormatType::{self, Zip},
};

pub struct ZipFormat {}

impl ArchiveFormat for ZipFormat {
    fn get_type(&self) -> ArchiveFormatType {
        Zip
    }
}
