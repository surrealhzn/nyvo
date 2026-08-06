use crate::MAGIC;
use aes_gcm_siv::{
    Aes256GcmSiv, Key, KeyInit,
    aead::{Aead, Generate},
};
use argon2::{Algorithm::Argon2id, Argon2, Version::V0x13};
use dh::{ReadVal, ReadValAt, WriteVal};
use rand::{Rng, rngs::ThreadRng};
use std::{
    error::Error,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

fn encode_ref(before: usize, current: usize, bits: u8) -> (u8, Option<usize>) {
    if current == 0 {
        return (0, None);
    }
    if current == before {
        return (1 << (bits - 1), None);
    }
    if current > before && ((current - before) < (1 << (bits - 1))) {
        let diff = (current - before) as u8;
        return (diff | (1 << (bits - 1)), None);
    }
    if current < before && ((before - current) < ((1 << (bits - 1)) - 1)) {
        let diff = (before - current) as u8;
        return (diff, None);
    }
    ((1 << (bits - 1)) - 1, Some(current))
}

#[derive(Copy, Clone)]
pub enum EncryptionAlgorithm {
    Aes256GcmSiv = 0,
}

impl TryFrom<u8> for EncryptionAlgorithm {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EncryptionAlgorithm::Aes256GcmSiv),
            _ => Err("Invalid encryption algorithm"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CompressionAlgorithm {
    None = 0,
    Zstd = 1,
}

impl TryFrom<u8> for CompressionAlgorithm {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CompressionAlgorithm::None),
            1 => Ok(CompressionAlgorithm::Zstd),
            _ => Err("Invalid compression algorithm"),
        }
    }
}

#[derive(Clone)]
pub struct EncryptionBuilder {
    algorithm: EncryptionAlgorithm,
    keys: Vec<KdfBuilder>,
    dek: [u8; 32],
    kdf_salt: [u8; 32],
    kdf_memory: u32,
    kdf_iterations: u32,
    kdf_parallelism: u32,
}

#[derive(Copy, Clone)]
pub struct KdfBuilder {
    nonce: [u8; 12],
    key: [u8; 32],
}

#[derive(Copy, Clone)]
pub struct StoreBuilder {
    encryption_method: EncryptionMethodRef,
    compression_method: CompressionAlgorithm,
    compression_level: u8,
}

impl Default for StoreBuilder {
    fn default() -> Self {
        Self {
            encryption_method: EncryptionMethodRef(None),
            compression_method: CompressionAlgorithm::None,
            compression_level: 0,
        }
    }
}

pub enum ContentBuilder<'a> {
    Index(IndexBuilder),
    Content(ContentBlockBuilder<'a>),
}

impl ContentBuilder<'_> {
    pub fn build(
        self,
        target: &mut dyn Write,
        options: (&StoreBuilder, Option<&EncryptionBuilder>),
        block_before: &mut usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut content = Cursor::new(vec![]);

        match self {
            ContentBuilder::Index(index) => index.build(&mut content, block_before)?,
            ContentBuilder::Content(cbb) => {
                for (mut cont, _) in cbb.content {
                    std::io::copy(&mut cont, &mut content)?;
                }
            }
        }

        // TODO: optimize this
        let mut content2: Cursor<Vec<u8>> = Cursor::new(vec![]);

        content.seek(SeekFrom::Start(0))?;
        match options.0.compression_method {
            CompressionAlgorithm::Zstd => {
                let mut encoder =
                    zstd::Encoder::new(&mut content2, options.0.compression_level as i32)?;
                std::io::copy(&mut content, &mut encoder)?;
                encoder.finish()?;
            }
            CompressionAlgorithm::None => {
                std::io::copy(&mut content, &mut content2)?;
            }
        }

        content2.seek(SeekFrom::Start(0))?;
        match options.1 {
            Some(enc) => match enc.algorithm {
                EncryptionAlgorithm::Aes256GcmSiv => {
                    let cipher = Aes256GcmSiv::new(&enc.dek.into());
                    let mut nonce = [0; 12];
                    let mut rng = ThreadRng::default();
                    rng.fill_bytes(&mut nonce);
                    let encrypted = cipher.encrypt(&nonce.into(), content2.get_ref().as_slice())?;
                    target.write_u8_array(nonce)?;
                    target.write_vec(encrypted)?;
                }
            },
            None => {
                std::io::copy(&mut content2, target)?;
            }
        }

        Ok(())
    }
}

pub struct ContentBlockBuilder<'a> {
    content: Vec<(&'a mut dyn Read, usize)>,
    length: usize,
}

pub struct ContentInfoBuilder<'a> {
    store_method: &'a StoreMethodRef,
    content: ContentBuilder<'a>,
}

impl ContentInfoBuilder<'_> {
    pub fn build(
        self,
        target: &mut dyn Write,
        store_method: &mut usize,
        options: (&StoreBuilder, Option<&EncryptionBuilder>),
        block_before: &mut usize,
    ) -> Result<(), Box<dyn Error>> {
        let is_index = match self.content {
            ContentBuilder::Index(_) => 1 << 7,
            ContentBuilder::Content(_) => 0,
        };
        let store_option_ref = encode_ref(*store_method, self.store_method.0, 7);
        *store_method = self.store_method.0;
        target.write_u8(store_option_ref.0 | is_index)?;
        if let Some(store_method) = store_option_ref.1 {
            target.write_vu8(store_method as _)?;
        }

        let mut content = Cursor::new(vec![]);
        self.content.build(&mut content, options, block_before)?;

        let len = content.seek(SeekFrom::End(0))?;
        target.write_vu8(len as _)?;
        let mut hasher = blake3::Hasher::new();
        content.seek(SeekFrom::Start(0))?;
        hasher.update_reader(&mut content)?;
        let mut hasher = hasher.finalize_xof();

        hasher.copy(
            if len < 0x1_00 {
                4 // < 256 B: 32 bit chksum
            } else if len < 0x1_00_00 {
                8 // 256 B - 64 kiB: 64 bit chksum
            } else if len < 0x1_00_00_00 {
                16 // 64 kiB - 16 MiB: 128 bit chksum
            } else {
                32 // > 16 MiB: 256 bit chksum
            },
            target,
        )?;

        content.copy_at(0, len, target)?;

        Ok(())
    }
}

pub struct IndexBuilder(Vec<IndexEntryBuilder>);

impl IndexBuilder {
    pub fn build(
        self,
        target: &mut dyn Write,
        block_before: &mut usize,
    ) -> Result<(), Box<dyn Error>> {
        for entry in self.0 {
            target.write_vu8(entry.path.len() as _)?;
            target.write_str(entry.path)?;
            let block_ref = encode_ref(*block_before, entry.block.0 as _, 8);
            target.write_u8(block_ref.0)?;
            if let Some(block) = block_ref.1 {
                target.write_vu8(block as _)?;
            }
            *block_before = entry.block.0;
            target.write_vu8(entry.offset as _)?;
            target.write_vu8(entry.length as _)?;
        }
        Ok(())
    }
}

pub struct IndexEntryBuilder {
    path: String,
    block: ContentRef,
    offset: usize,
    length: usize,
}

pub struct ArchiveBuilder<'a> {
    version: u8,
    encryption_methods: Vec<EncryptionBuilder>,
    store_methods: Vec<StoreBuilder>,
    content: Vec<ContentInfoBuilder<'a>>,
    block_refs: Vec<usize>,
}

pub struct IndexRef(usize);
pub struct ContentRef(usize);
pub struct InnerContentRef(ContentRef, usize, usize);

pub struct StoreMethodRef(usize);

const STORE_METHOD_REF_DEFAULT: StoreMethodRef = StoreMethodRef(0);
impl Default for &StoreMethodRef {
    fn default() -> Self {
        &STORE_METHOD_REF_DEFAULT
    }
}

#[derive(Copy, Clone, Default)]
pub struct EncryptionMethodRef(Option<usize>);

impl From<EncryptionMethodRef> for u128 {
    fn from(value: EncryptionMethodRef) -> Self {
        value.0.map(|v| v as u128 + 1).unwrap_or(0)
    }
}

impl<'a> ArchiveBuilder<'a> {
    pub fn new() -> Self {
        Self {
            version: 1,
            encryption_methods: vec![],
            store_methods: vec![],
            content: vec![],
            block_refs: vec![],
        }
    }

    pub fn encrypt(&mut self, key: &[u8]) -> EncryptionMethodRef {
        let kdf_memory = 1 << 16;
        let kdf_iterations = 3;
        let kdf_parallelism = 1;

        let hasher = Argon2::new(
            Argon2id,
            V0x13,
            argon2::Params::new(kdf_memory, kdf_iterations, kdf_parallelism, None).unwrap(),
        );

        let mut rng = ThreadRng::default();
        let salt: [u8; 32] = Key::<Aes256GcmSiv>::generate_from_rng(&mut rng).into();
        let mut kek = [0; 32];
        hasher.hash_password_into(key, &salt, &mut kek).unwrap();

        let mut nonce = [0; 12];
        rng.fill_bytes(&mut nonce);

        self.encryption_methods.push(EncryptionBuilder {
            algorithm: EncryptionAlgorithm::Aes256GcmSiv,
            keys: vec![KdfBuilder { nonce, key: kek }],
            dek: Key::<Aes256GcmSiv>::generate_from_rng(&mut rng).into(),
            kdf_memory,
            kdf_iterations,
            kdf_parallelism,
            kdf_salt: salt,
        });
        EncryptionMethodRef(Some(self.encryption_methods.len() - 1))
    }

    pub fn add_store_method(
        &mut self,
        encryption_method: EncryptionMethodRef,
        compression_method: CompressionAlgorithm,
        compression_level: u8,
    ) -> StoreMethodRef {
        self.store_methods.push(StoreBuilder {
            encryption_method,
            compression_method,
            compression_level,
        });
        StoreMethodRef(self.store_methods.len() - 1)
    }

    pub fn add_index(&mut self, store_method: &'a StoreMethodRef) -> IndexRef {
        self.content.push(ContentInfoBuilder {
            store_method,
            content: ContentBuilder::Index(IndexBuilder(vec![])),
        });
        IndexRef(self.content.len() - 1)
    }

    pub fn add_content_block(&mut self, store_method: &'a StoreMethodRef) -> ContentRef {
        let block = self.block_refs.len();
        self.block_refs.push(self.content.len());
        self.content.push(ContentInfoBuilder {
            store_method,
            content: ContentBuilder::Content(ContentBlockBuilder {
                content: vec![],
                length: 0,
            }),
        });
        ContentRef(block)
    }

    pub fn add_content(
        &mut self,
        block: ContentRef,
        source: &'a mut dyn Read,
        length: usize,
    ) -> InnerContentRef {
        if let Some(ContentInfoBuilder {
            content: ContentBuilder::Content(cbb),
            ..
        }) = self.content.get_mut(self.block_refs[block.0])
        {
            cbb.content.push((source, length));
            let offset = cbb.length;
            cbb.length += length;
            InnerContentRef(block, offset, length)
        } else {
            todo!(); // ContentRef always points to a content block, not an index UNLESS the user pushes a foreign ContentRef, which unfortunately cannot be prevented
        }
    }

    pub fn index(&mut self, block: IndexRef, path: String, content: InnerContentRef) {
        if let Some(ContentInfoBuilder {
            content: ContentBuilder::Index(index),
            ..
        }) = self.content.get_mut(block.0)
        {
            index.0.push(IndexEntryBuilder {
                path,
                block: content.0,
                offset: content.1,
                length: content.2,
            });
        } else {
            todo!(); // IndexRef always points to an index, not a content block UNLESS the user pushes a foreign IndexRef, which unfortunately cannot be prevented
        }
    }

    pub fn build(self, target: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        target.write_u8_array(MAGIC)?;
        target.write_vu8((self.version - 1) as _)?;
        target.write_vu8(self.encryption_methods.len() as _)?;
        target.write_vu8(self.store_methods.len() as _)?;

        for method in self.encryption_methods.clone() {
            target.write_vu8(method.algorithm as _)?;
            target.write_vu8(method.kdf_memory as _)?;
            target.write_vu8(method.kdf_iterations as _)?;
            target.write_vu8(method.kdf_parallelism as _)?;
            target.write_u8_array(method.kdf_salt)?;
            target.write_vu8(method.keys.len() as _)?;

            for key in method.keys {
                target.write_u8_array(key.nonce)?;

                target.write_u8_array::<48>(
                    Aes256GcmSiv::new(&key.key.into())
                        .encrypt(&key.nonce.into(), method.dek.as_ref())?
                        .try_into()
                        .unwrap(),
                )?;
            }
        }

        for method in self.store_methods.clone() {
            target.write_vu8(method.encryption_method.into())?;
            target.write_vu8(method.compression_method as _)?;
        }

        let mut store_method = 0;
        let mut block_before = 0;
        for content in self.content {
            let store_option = if self.store_methods.is_empty() && content.store_method.0 == 0 {
                &StoreBuilder::default()
            } else {
                self.store_methods
                    .get(content.store_method.0)
                    .ok_or("Invalid store method")?
            };
            let encryption_option = store_option
                .encryption_method
                .0
                .map(|idx| {
                    self.encryption_methods
                        .get(idx)
                        .ok_or("Invalid encryption method")
                })
                .transpose()?;
            let options = (store_option, encryption_option);
            content.build(target, &mut store_method, options, &mut block_before)?;
        }

        Ok(())
    }
}

impl Default for ArchiveBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
