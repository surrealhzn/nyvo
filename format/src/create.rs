use crate::{MAGIC, Result};
use aes_gcm_siv::{
    Aes256GcmSiv, Nonce,
    aead::{Aead, KeyInit},
};
use dh::WriteVal;
use std::io::{Read, Seek, Write};

#[derive(Debug)]
pub struct CreateArchive {
    pub version: u16,
    pub encryption_methods: Vec<EncryptionMethod>,
    pub store_options: Vec<StoreOption>,
    pub indexes: Vec<Index>,
}

#[derive(Debug)]
pub struct EncryptionMethod {
    pub algorithm: EncryptionAlgorithm,
    pub kdf_memory: u32,
    pub kdf_iterations: u32,
    pub kdf_parallel: u32,
    pub dek: [u8; 32],
    pub keys: Vec<EncryptionKey>,
}

#[derive(Debug)]
#[repr(u8)]
pub enum EncryptionAlgorithm {
    Aes256GcmSiv = 0,
}

impl From<u8> for EncryptionAlgorithm {
    fn from(value: u8) -> Self {
        use EncryptionAlgorithm::*;
        match value {
            0 => Aes256GcmSiv,
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct EncryptionKey {
    pub kdf_salt: [u8; 32],
    pub nonce: [u8; 12],
    pub key: [u8; 32],
}

#[derive(Debug)]
pub struct StoreOption {
    pub encryption_method: Option<u32>,
    pub compression_method: CompressionMethod,
    pub compression_level: u8,
}

pub const STORE_OPTION_DEFAULT: StoreOption = StoreOption {
    encryption_method: None,
    compression_method: CompressionMethod::None,
    compression_level: 0,
};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CompressionMethod {
    None = 0,
    Zstd = 1,
}

#[derive(Debug)]
pub struct Index {
    pub last_here: bool, // whether a data block will follow this index (=1) or another index (=0)
    pub last_total: bool, // whether this is the last index in the archive
    pub index_size_varint: bool, // whether to store the index size as a variable-length integer
    pub store_option: StoreOptionReference,
    pub entries: Vec<IndexEntry>,
}

#[derive(Debug)]
pub struct StoreOptionReference(StoreOptionReferenceVariant, usize); // usize limit because of max Vec<T> length, .1 is only used when .0 is Custom

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum StoreOptionReferenceVariant {
    Default = 0,
    Custom = 1,
    Previous = 2,
    Increment = 3,
    Decrement = 4,
    Merge = 5,
}

#[derive(Debug)]
pub struct IndexEntry {}

pub fn create<T: Read + Write + Seek>(
    archive: CreateArchive,
    mut target: &mut dyn Write,
    mut cache: &mut T,
) -> Result<()> {
    // main header
    target.write_u8_array(MAGIC)?; // magic value
    target.write_vu8(archive.version as u128 - 1)?; // format version
    target.write_vu8(archive.encryption_methods.len() as u128)?; // encryption method count
    target.write_vu8(archive.store_options.len() as u128)?; // store option count

    // encryption methods
    for method in archive.encryption_methods {
        target.write_vu8(method.algorithm as u128)?; // encryption algorithm
        target.write_u32_le(method.kdf_memory)?; // KDF memory
        target.write_u32_le(method.kdf_iterations)?; // KDF iterations
        target.write_u32_le(method.kdf_parallel)?; // KDF parallel
        target.write_vu8(method.keys.len() as u128)?; // key count

        // keys
        for key in method.keys {
            target.write_u8_array(key.kdf_salt)?; // KDF salt
            target.write_u8_array(key.nonce)?; // nonce

            target.write_u8_array::<48>(
                // DEK ciphertext
                Aes256GcmSiv::new(&key.key.into())
                    .encrypt(Nonce::from_slice(&key.nonce), method.dek.as_ref())?
                    .try_into()
                    .unwrap(), // this should always be 48 bytes, there is no case in which it is more or less
            )?;
        }
    }

    // store options
    for option in archive.store_options {
        target.write_vu8(match option.encryption_method {
            // encryption method id
            None => 0,
            Some(id) => id + 1,
        } as u128)?;
        target.write_vu8(option.compression_method as u128)?; // compression method
        if option.compression_method != CompressionMethod::None {
            target.write_vu8(option.compression_level as u128)?;
        }
    }

    // content
    let mut current_index: usize = 0;
    let mut current_file: usize = 0;
    for index in archive.indexes {
        target.write_u8(
            index.store_option.0 as u8
                | (index.last_here as u8) << 7
                | (index.last_total as u8) << 6
                | (index.index_size_varint as u8) << 5,
        )?;
        if index.store_option.0 == StoreOptionReferenceVariant::Custom {
            target.write_vu8(index.store_option.1 as u128)?;
        }
    }

    Ok(())
}
