pub use super::{
    ArchiveFormat,
    ArchiveFormatType::{self, Nyvo},
};

pub struct NyvoFormat {}

impl ArchiveFormat for NyvoFormat {
    fn get_type(&self) -> ArchiveFormatType {
        Nyvo
    }
}
