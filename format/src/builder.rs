use crate::MAGIC;
use aes_gcm_siv::{
    Aes256GcmSiv, KeyInit, Nonce,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use argon2::{Algorithm::Argon2id, Argon2, Version::V0x13};
use dh::WriteVal;
use std::{
    error::Error,
    io::{Seek, Write},
};

pub enum EncryptionAlgorithm {
    Aes256GcmSiv = 0,
}

pub enum CompressionAlgorithm {
    None = 0,
    Zstd = 1,
}

pub struct EncryptionBuilder {
    algorithm: EncryptionAlgorithm,
    keys: Vec<KdfBuilder>,
    dek: [u8; 32],
    kdf_memory: u32,
    kdf_iterations: u32,
    kdf_parallelism: u32,
}

pub struct KdfBuilder {
    salt: [u8; 32],
    nonce: [u8; 12],
    key: [u8; 32],
}

pub struct StoreBuilder {
    encryption_method: usize,
    compression_method: CompressionAlgorithm,
    compression_level: u8,
}

pub struct ArchiveBuilder {
    version: u8,
    encryption_methods: Vec<EncryptionBuilder>,
    store_methods: Vec<StoreBuilder>,
}

impl ArchiveBuilder {
    pub fn new() -> Self {
        Self {
            version: 1,
            encryption_methods: vec![],
            store_methods: vec![],
        }
    }

    pub fn encrypt(&mut self, key: &[u8]) -> usize {
        let kdf_memory = 1 << 16;
        let kdf_iterations = 3;
        let kdf_parallelism = 1;

        let hasher = Argon2::new(
            Argon2id,
            V0x13,
            argon2::Params::new(kdf_memory, kdf_iterations, kdf_parallelism, None).unwrap(),
        );

        let salt: [u8; 32] = Aes256GcmSiv::generate_key(&mut OsRng).into();
        let mut kek = [0; 32];
        hasher.hash_password_into(key, &salt, &mut kek).unwrap();

        let mut nonce = [0; 12];
        OsRng.fill_bytes(&mut nonce);

        self.encryption_methods.push(EncryptionBuilder {
            algorithm: EncryptionAlgorithm::Aes256GcmSiv,
            keys: vec![KdfBuilder {
                salt,
                nonce,
                key: kek,
            }],
            dek: Aes256GcmSiv::generate_key(&mut OsRng).into(),
            kdf_memory,
            kdf_iterations,
            kdf_parallelism,
        });
        self.encryption_methods.len()
    }

    pub fn build<T: Write + Seek>(self, mut target: T) -> Result<(), Box<dyn Error>> {
        target.write_u8_array(MAGIC)?;
        target.write_vu8(self.version as _)?;
        target.write_vu8(self.encryption_methods.len() as _)?;
        target.write_vu8(self.store_methods.len() as _)?;

        for method in self.encryption_methods {
            target.write_vu8(method.algorithm as _)?;
            target.write_vu8(method.kdf_memory as _)?;
            target.write_vu8(method.kdf_iterations as _)?;
            target.write_vu8(method.kdf_parallelism as _)?;
            target.write_vu8(method.keys.len() as _)?;

            for key in method.keys {
                target.write_u8_array(key.salt)?;
                target.write_u8_array(key.nonce)?;

                target.write_u8_array::<48>(
                    Aes256GcmSiv::new(&key.key.into())
                        .encrypt(Nonce::from_slice(&key.nonce), method.dek.as_ref())?
                        .try_into()
                        .unwrap(),
                )?;
            }
        }

        Ok(())
    }
}

impl Default for ArchiveBuilder {
    fn default() -> Self {
        Self::new()
    }
}
