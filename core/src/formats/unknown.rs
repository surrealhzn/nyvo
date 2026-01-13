pub use super::{
    ArchiveFormat,
    ArchiveFormatType::{self, Unknown},
};

pub struct UnknownFormat {}

impl ArchiveFormat for UnknownFormat {
    fn get_type(&self) -> ArchiveFormatType {
        Unknown
    }
}
