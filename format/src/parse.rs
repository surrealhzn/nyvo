use crate::{EncryptionAlgorithm, EncryptionKey, EncryptionMethod, Result, StoreOption};
use dh::{ReadSeek, ReadVal};

pub struct ParsedBlocks {
    pub version: u16,
    pub encryption_methods: Vec<EncryptionMethod>,
    pub store_options: Vec<StoreOption>,
}

pub fn parse_blocks(mut source: &mut dyn ReadSeek) -> Result<ParsedBlocks> {
    source.seek_relative(8)?; // magic
    let version = source.read_vu8()? as u16;

    let encryption_method_count = source.read_vu8()? as usize;
    let store_option_count = source.read_vu8()? as usize;

    let mut encryption_methods = vec![];
    let mut store_options = vec![];

    while encryption_method_count > encryption_methods.len() {
        let algorithm = (source.read_vu8()? as u8).into();
        let kdf_memory = source.read_u32_le()?;
        let kdf_iterations = source.read_u32_le()?;
        let kdf_parallel = source.read_u32_le()?;
        let key_count = source.read_vu8()? as usize;

        let mut keys = vec![];

        while key_count > keys.len() {
            let kdf_salt = source.read_u8_array()?;
            let nonce = source.read_u8_array()?;
            source.seek_relative(48)?; // encrypted dek

            keys.push(EncryptionKey {
                kdf_salt,
                nonce,
                key: [0; 32],
            })
        }

        encryption_methods.push(EncryptionMethod {
            algorithm,
            kdf_memory,
            kdf_iterations,
            kdf_parallel,
            dek: [0; 32],
            keys,
        });
    }

    Ok(ParsedBlocks {
        version,
        encryption_methods,
        store_options,
    })
}
