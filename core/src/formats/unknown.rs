use dh::ReadSeek;

use crate::{
    Res,
    env::Env,
    formats::{Blocks, Index},
};

pub use super::{
    ArchiveFormat,
    ArchiveFormatType::{self, Unknown},
};

pub struct UnknownFormat {}

impl ArchiveFormat<'_> for UnknownFormat {
    fn new(env: Env, source: &mut dyn dh::ReadSeek) -> Self {
        todo!()
    }

    fn get_type(&self) -> ArchiveFormatType {
        Unknown
    }

    fn add_key(&mut self, key: &[u8]) {}

    fn index_blocks(&mut self) -> Res<&Blocks> {
        todo!()
    }

    fn index(&mut self) -> Res<&Index> {
        todo!()
    }

    fn extract_block(&mut self, block: usize, target: &mut dyn std::io::Write) -> Res<()> {
        todo!()
    }
    fn extract_file(
        &self,
        file: usize,
        block: &mut dyn ReadSeek,
        target: &mut dyn std::io::Write,
    ) -> Res<()> {
        todo!();
    }
}
