use std::{io::Write, path::PathBuf};

use dh::ReadValAt;

use crate::{Res, Rs, env::Env};

#[cfg(feature = "nyvo")]
pub mod nyvo;
#[cfg(feature = "zip")]
pub mod zip;

pub mod unknown;

pub enum ArchiveFormatType {
    #[cfg(feature = "nyvo")]
    Nyvo,
    #[cfg(feature = "zip")]
    Zip,

    Unknown,
}

#[derive(Debug)]
pub struct Block {
    pub offset: u64,
    pub size: u64,
    pub encryption_id: Option<usize>,
    pub compression_id: Option<usize>,
}
pub type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct IndexItem {
    pub path: PathBuf,
    pub block: usize,
    pub offset: u64,
    pub size: u64,
}
pub type Index = Vec<IndexItem>;

pub trait ArchiveFormat<'a> {
    fn new(env: Env, source: &'a mut dyn Rs) -> Self;
    fn get_type(&self) -> ArchiveFormatType;

    fn compression_methods(&self) -> Vec<CompressionMethod> {
        vec![]
    }

    fn encryption_methods(&self) -> Vec<EncryptionMethod> {
        vec![]
    }

    fn add_key(&mut self, key: &[u8]);
    fn index_blocks(&mut self) -> Res<&Blocks>;
    fn index(&mut self) -> Res<&Index>;

    fn extract_block(&mut self, block: usize, target: &mut dyn Write) -> Res<()>;
    fn extract_file(
        &self,
        file: usize,
        block: &mut dyn Rs,
        target: &mut dyn std::io::Write,
    ) -> Res<()>;
}

pub enum EncryptionAlgorithm {}

pub struct EncryptionMethod {
    pub algorithm: EncryptionAlgorithm,
    pub key: Vec<u8>,
}

#[derive(Debug)]
pub enum CompressionAlgorithm {
    Unknown,
    Deflate,
}

#[derive(Debug)]
pub struct CompressionMethod {
    pub algorithm: CompressionAlgorithm,
    pub level: Option<u8>,
}

impl<'a> CompressionMethod {
    pub fn unknown() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Unknown,
            level: None,
        }
    }

    pub fn unknown_ref() -> &'a Self {
        &Self {
            algorithm: CompressionAlgorithm::Unknown,
            level: None,
        }
    }
}

pub fn extract(
    mut source: &mut dyn Rs,
    offset: u64,
    size: u64,
    target: &mut dyn Write,
    encryption: Option<&EncryptionMethod>,
    compression: Option<&CompressionMethod>,
) -> Res<()> {
    if encryption.is_some() {
        todo!();
    }

    if let Some(compression) = compression {
        match compression.algorithm {
            #[cfg(feature = "deflate")]
            CompressionAlgorithm::Deflate => {
                use dh::ReadVal;
                use flate2::read::DeflateDecoder;
                let mut decoder = DeflateDecoder::new(source);
                decoder.copy(size, target)?;
                return Ok(());
            }
            _ => todo!(),
        }
    }

    source.copy_at(offset, size, target)?;
    Ok(())
}
