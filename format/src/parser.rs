use crate::{CompressionAlgorithm, EncryptionAlgorithm, MAGIC};
use dh::{ReadVal, helpers::Rs};
use std::error::Error;

struct EncryptionMethod {
    algorithm: EncryptionAlgorithm,
    kdf_memory: u32,
    kdf_iterations: u32,
    kdf_parallelism: u32,
    kdf_salt: [u8; 32],
    deks: Vec<DataEncryptionKey>,
}

struct DataEncryptionKey {
    nonce: [u8; 12],
    cipher: [u8; 48],
}

struct StoreMethod {
    encryption: usize,
    compression: CompressionAlgorithm,
}

impl Default for StoreMethod {
    fn default() -> Self {
        Self {
            encryption: 0,
            compression: CompressionAlgorithm::None,
        }
    }
}

pub fn load_archive(source: &mut dyn Rs) -> Result<(), Box<dyn Error>> {
    if source.read_u8_array()? != MAGIC {
        return Err("magic mismatch".into());
    };
    let version = source.read_vu8()? + 1;
    if version != 1 {
        return Err("unsupported version".into());
    }

    let encryption_methods_len = source.read_vu8()? as _;
    let store_methods_len = source.read_vu8()? as usize;

    let mut encryption_methods = Vec::with_capacity(encryption_methods_len);
    for _ in 0..encryption_methods_len {
        let algorithm = (source.read_vu8()? as u8).try_into()?;
        let kdf_memory = source.read_vu8()? as _;
        let kdf_iterations = source.read_vu8()? as _;
        let kdf_parallelism = source.read_vu8()? as _;
        let kdf_salt = source.read_u8_array()?;

        let deks_len = source.read_vu8()? as usize;
        let mut deks = Vec::with_capacity(deks_len);
        for _ in 0..deks_len {
            let nonce = source.read_u8_array()?;
            let cipher = source.read_u8_array()?;
            deks.push(DataEncryptionKey { nonce, cipher });
        }

        encryption_methods.push(EncryptionMethod {
            algorithm,
            kdf_memory,
            kdf_iterations,
            kdf_parallelism,
            kdf_salt,
            deks,
        });
    }

    let store_methods = if store_methods_len == 0 {
        vec![StoreMethod::default()]
    } else {
        let mut store_methods = Vec::with_capacity(store_methods_len);
        for _ in 0..store_methods_len {
            let encryption = source.read_vu8()? as _;
            let compression = (source.read_vu8()? as u8).try_into()?;
            store_methods.push(StoreMethod {
                encryption,
                compression,
            });
        }
        store_methods
    };

    todo!();

    Ok(())
}
