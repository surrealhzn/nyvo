use std::path::PathBuf;

use dh::ReadValAt;
use zip::{CompressionMethod, ZipArchive};

use crate::{
    Err, Res, Rs,
    env::Env,
    formats::{Blocks, IndexItem},
    unwrap_return_ref,
};

pub use super::{
    ArchiveFormat,
    ArchiveFormatType::{self, Zip},
    Index,
};

pub struct ZipFormat<'a> {
    source: &'a mut dyn Rs,
    keys: Vec<Vec<u8>>,
    index: Option<Index>,
    blocks: Option<Blocks>,
    compression_methods: Vec<super::CompressionMethod>,
}

impl<'a> ArchiveFormat<'a> for ZipFormat<'a> {
    fn new(env: Env, source: &'a mut dyn Rs) -> Self {
        Self {
            source,
            index: None,
            blocks: None,
            keys: vec![],
            compression_methods: vec![],
        }
    }

    fn get_type(&self) -> ArchiveFormatType {
        Zip
    }

    fn add_key(&mut self, key: &[u8]) {
        self.keys.push(key.to_vec());
    }

    fn index_blocks(&mut self) -> Res<&Blocks> {
        unwrap_return_ref!(self.blocks);

        let mut archive = ZipArchive::new(&mut self.source)?;
        let mut index = vec![];
        let mut blocks = vec![];
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            blocks.push(crate::formats::Block {
                offset: file.data_start().unwrap(), // why is this even an option??
                size: file.compressed_size(),
                encryption_id: None, // TODO
                compression_id: match file.compression() {
                    CompressionMethod::Stored => None,
                    CompressionMethod::Deflated => {
                        self.compression_methods.push(super::CompressionMethod {
                            algorithm: super::CompressionAlgorithm::Deflate,
                            level: None,
                        });
                        Some(self.compression_methods.len() - 1)
                    }
                    _ => {
                        self.compression_methods
                            .push(super::CompressionMethod::unknown());
                        Some(self.compression_methods.len() - 1)
                    }
                },
            });
            index.push(IndexItem {
                path: PathBuf::from(file.name()),
                block: i,
                offset: 0,
                size: file.size(),
            });
        }
        self.index = Some(index);

        self.blocks = Some(blocks);
        self.blocks.as_ref().ok_or(Err::Safe)
    }

    fn index(&mut self) -> Res<&Index> {
        unwrap_return_ref!(self.index);

        self.index_blocks()?;
        self.index.as_ref().ok_or(Err::Safe)
    }

    fn extract_block(&mut self, block: usize, target: &mut dyn std::io::Write) -> Res<()> {
        let block = self
            .blocks
            .as_ref()
            .ok_or(Err::ExecOrder)?
            .get(block)
            .ok_or(Err::NotFoundInArchive(format!("#blk-{:x}", block)))?;

        super::extract(
            &mut self.source,
            block.offset,
            block.size,
            target,
            None, // TODO
            if let Some(id) = block.compression_id {
                self.compression_methods
                    .get(id)
                    .or(Some(super::CompressionMethod::unknown_ref()))
            } else {
                None
            },
        )?;

        Ok(())
    }

    fn extract_file(
        &self,
        file: usize,
        mut block: &mut dyn Rs,
        target: &mut dyn std::io::Write,
    ) -> Res<()> {
        let file = self
            .index
            .as_ref()
            .ok_or(Err::ExecOrder)?
            .get(file)
            .ok_or(Err::NotFoundInArchive(format!("#file-{:x}", file)))?;

        block.copy_at(file.offset, file.size, target)?;
        Ok(())
    }
}
