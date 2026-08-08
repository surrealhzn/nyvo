use crate::{CompressionAlgorithm, EncryptionAlgorithm, MAGIC};
use aes_gcm_siv::{Aes256GcmSiv, KeyInit, aead::Aead};
use argon2::{Algorithm::Argon2id, Argon2, Version::V0x13};
use dh::{ReadVal, helpers::Rs};
use std::{
    collections::HashMap,
    error::Error,
    io::{Cursor, Read, SeekFrom, Write},
};

fn decode_ref(before: usize, value: u8, bits: u8) -> Option<usize> {
    if value == 0 {
        return Some(0);
    }
    if value == (1 << (bits - 1)) {
        return Some(before);
    }
    if value > (1 << (bits - 1)) {
        let diff = (value & ((1 << (bits - 1)) - 1)) as usize;
        return Some(before + diff);
    }
    let diff = value as usize;
    if before < diff {
        return None;
    }
    Some(before - diff)
}

pub struct EncryptionMethod {
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

#[derive(Debug, PartialEq, Eq)]
pub struct StoreMethod {
    pub encryption: usize,
    pub compression: CompressionAlgorithm,
}

impl Default for StoreMethod {
    fn default() -> Self {
        Self {
            encryption: 0,
            compression: CompressionAlgorithm::None,
        }
    }
}

pub struct LoadedArchive<'a> {
    reader: &'a mut dyn Rs,
    pub encryption_methods: Vec<EncryptionMethod>,
    pub store_methods: Vec<StoreMethod>,
    pub content: Vec<ContentInfo>,
    unlocked_deks: Vec<Option<[u8; 32]>>,
}

pub struct ContentInfo {
    pub is_index: bool,
    pub store_method: usize,
    pub hash: Vec<u8>,
    pub offset: u64,
    pub len: u64,
}

impl LoadedArchive<'_> {
    pub fn unlock(&mut self, key: &[u8]) -> Result<bool, Box<dyn Error>> {
        let mut unlocked = false;
        for (i, method) in self.encryption_methods.iter().enumerate() {
            if self.unlocked_deks[i].is_some() {
                continue;
            }
            let hasher = Argon2::new(
                Argon2id,
                V0x13,
                argon2::Params::new(
                    method.kdf_memory,
                    method.kdf_iterations,
                    method.kdf_parallelism,
                    None,
                )
                .unwrap(),
            );
            let mut kek = [0u8; 32];
            hasher
                .hash_password_into(key, &method.kdf_salt, &mut kek)
                .unwrap();

            for dek in &method.deks {
                let cipher = Aes256GcmSiv::new(&kek.into());
                if let Ok(plain) = cipher.decrypt(&dek.nonce.into(), dek.cipher.as_ref()) {
                    self.unlocked_deks[i] = Some(plain.try_into().unwrap());
                    unlocked = true;
                    break;
                } else {
                    continue;
                }
            }
        }
        Ok(unlocked)
    }

    pub fn check_integrity(&mut self) -> Result<bool, Box<dyn Error>> {
        for content in &self.content {
            let hash = &content.hash;

            self.reader.seek(SeekFrom::Start(content.offset))?;
            let mut hasher = blake3::Hasher::new();
            let mut reader = self.reader.take(content.len);
            hasher.update_reader(&mut reader)?;
            let mut hasher = hasher.finalize_xof();
            let mut computed_hash = vec![0u8; hash.len()];
            hasher.fill(&mut computed_hash);
            if hash != &computed_hash {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

pub fn load_archive<'a>(source: &'a mut dyn Rs) -> Result<LoadedArchive<'a>, Box<dyn Error>> {
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
            if encryption > encryption_methods_len {
                return Err("invalid encryption ref".into());
            }
            let compression = (source.read_vu8()? as u8).try_into()?;
            store_methods.push(StoreMethod {
                encryption,
                compression,
            });
        }
        store_methods
    };

    let mut store_method = 0;
    let mut content = vec![];
    while let Ok(store_option_ref) = source.read_u8() {
        let is_index = store_option_ref & (1 << 7) != 0;
        store_method = if let Some(opt) = decode_ref(store_method, store_option_ref, 7) {
            opt
        } else {
            source.read_vu8()? as _
        };
        if store_methods.len() <= store_method {
            return Err("invalid store method ref".into());
        }
        let len = source.read_vu8()?;
        let hash = source.read_vec(if len < 0x1_00 {
            4
        } else if len < 0x1_00_00 {
            8
        } else if len < 0x1_00_00_00 {
            16
        } else {
            32
        })?;
        let offset = source.stream_position()?;
        content.push(ContentInfo {
            is_index,
            store_method,
            hash,
            offset,
            len: len as _,
        });
        source.seek(SeekFrom::Current(len as _))?;
    }

    Ok(LoadedArchive {
        reader: source,
        encryption_methods,
        store_methods,
        content,
        unlocked_deks: vec![None; encryption_methods_len],
    })
}

pub struct IndexedArchive<'a> {
    reader: &'a mut dyn Rs,
    pub encryption_methods: Vec<EncryptionMethod>,
    pub store_methods: Vec<StoreMethod>,
    pub blocks: Vec<(usize, usize, usize)>, // (offset, length, store_method)
    pub index: HashMap<String, IndexEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IndexEntry {
    pub block: usize,
    pub offset: usize,
    pub len: usize,
}

impl IndexedArchive<'_> {
    pub fn extract<'a>(
        &mut self,
        paths: Vec<&str>,
        writer: impl FnMut(&str) -> Box<dyn Write>,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: optimize by sorting files by block
        for path in paths {}
        Ok(())
    }
}

impl<'a> TryFrom<LoadedArchive<'a>> for IndexedArchive<'a> {
    type Error = Box<dyn Error>;

    fn try_from(loaded: LoadedArchive<'a>) -> Result<Self, Self::Error> {
        let reader = loaded.reader;
        let encryption_methods = loaded.encryption_methods;
        let store_methods = loaded.store_methods;
        let mut index = HashMap::new();

        for idx in loaded.content.iter().filter(|c| c.is_index) {
            let store_method = store_methods.get(idx.store_method).unwrap(); // checked during load_archive
            reader.seek(SeekFrom::Start(idx.offset))?;
            let mut reader: Box<dyn Read> = Box::new(reader.take(idx.len));

            if store_method.encryption != 0 {
                let encryption_id = store_method.encryption - 1; // valid, checked during load_archive
                let dek = if let Some(dek) = loaded.unlocked_deks.get(encryption_id).unwrap() {
                    dek
                } else {
                    continue;
                };

                match encryption_methods.get(encryption_id).unwrap().algorithm {
                    EncryptionAlgorithm::Aes256GcmSiv => {
                        let nonce = reader.read_u8_array()?;
                        let cipher = Aes256GcmSiv::new(dek.into());
                        let encrypted = reader.read_vec(idx.len as usize - 12)?;
                        let decrypted = cipher.decrypt(&nonce.into(), encrypted.as_ref())?;
                        reader = Box::new(Cursor::new(decrypted));
                    }
                }
            }

            match store_method.compression {
                CompressionAlgorithm::None => {}
                CompressionAlgorithm::Zstd => {
                    reader = Box::new(zstd::Decoder::new(reader)?);
                }
            }

            loop {
                let path_len = match reader.read_vu8() {
                    Ok(len) => len,
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(e.into()),
                };

                let path = reader.read_str(path_len as _)?;
                let block = reader.read_vu8()? as _;
                let offset = reader.read_vu8()? as _;
                let len = reader.read_vu8()? as _;
                index.insert(path, IndexEntry { block, offset, len });
            }
        }

        let blocks = loaded
            .content
            .iter()
            .filter(|c| !c.is_index)
            .map(|c| (c.offset as usize, c.len as usize, c.store_method))
            .collect::<Vec<_>>();

        Ok(Self {
            reader,
            index,
            encryption_methods,
            store_methods,
            blocks,
        })
    }
}
