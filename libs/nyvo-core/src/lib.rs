pub mod compression;
pub mod env;
pub mod formats;

mod error;

pub use error::*;

pub(crate) mod helpers;

mod types;
pub use types::*;

pub enum ArchiveFeature {
    Create,
}

pub enum ArchiveFormat {
    Nyvo,
    Zip,
}

pub enum CompressionAlgorithm {
    None,
}

pub struct ConvertOptions {
    pub format: ArchiveFormat,
    pub optimize: OptimizationMode,
    pub encoding_rewriter: Option<Box<dyn Fn(&mut ArchiveEntry) -> &mut ArchiveEntry + 'static>>,
}

pub struct ConvertOptionsBuilder {
    format: ArchiveFormat,
    optimize: OptimizationMode,
    encoding_rewriter: Option<Box<dyn Fn(&mut ArchiveEntry) -> &mut ArchiveEntry + 'static>>,
}

impl ConvertOptionsBuilder {
    pub fn new() -> Self {
        ConvertOptionsBuilder {
            format: ArchiveFormat::Nyvo,
            optimize: OptimizationMode::WriteSpeed,
            encoding_rewriter: None,
        }
    }

    pub fn format(mut self, format: ArchiveFormat) -> Self {
        self.format = format;
        self
    }

    pub fn optimize(mut self, optimize: OptimizationMode) -> Self {
        self.optimize = optimize;
        self
    }

    pub fn encoding_rewriter<F>(mut self, rewriter: F) -> Self
    where
        F: Fn(&mut ArchiveEntry) -> &mut ArchiveEntry + 'static,
    {
        self.encoding_rewriter = Some(Box::new(rewriter));
        self
    }

    pub fn build(self) -> ConvertOptions {
        ConvertOptions {
            format: self.format,
            optimize: self.optimize,
            encoding_rewriter: self.encoding_rewriter,
        }
    }
}

pub struct ArchiveEntry {
    pub path: std::path::PathBuf,
    pub compression: CompressionAlgorithm,
}

impl ArchiveEntry {
    pub fn compression(&mut self, compression: CompressionAlgorithm) -> &mut Self {
        self.compression = compression;
        self
    }
}

pub enum OptimizationMode {
    WriteSpeed,
    ReadSpeed,
}

pub struct NyvoCore;

impl NyvoCore {
    pub fn new() -> Self {
        NyvoCore
    }

    pub fn extract(&self, source: NyvoSource, output_dir: &str) -> Result<(), ()> {
        todo!()
    }

    pub fn open(&self, source: NyvoSource) -> Result<Archive, ()> {
        todo!()
    }

    pub fn create(&self, format: ArchiveFormat) -> Archive {
        todo!()
    }
}

pub struct Archive;

impl Archive {
    pub fn index(&mut self) -> Result<(), ()> {
        todo!()
    }

    pub fn select_files<F>(&self, predicate: F) -> Result<Vec<ArchiveEntry>, ()>
    where
        F: Fn(&ArchiveEntry) -> bool,
    {
        todo!()
    }

    pub fn extract(&self, files: Vec<ArchiveEntry>, output_dir: &str) -> Result<(), ()> {
        todo!()
    }

    pub fn add_file(&mut self, source: NyvoSource, path_in_archive: &str) {
        todo!()
    }

    pub fn build(&self, output_path: &str) -> Result<(), ()> {
        todo!()
    }

    pub fn convert(&self, options: ConvertOptions, output_path: &str) -> Result<(), ()> {
        todo!()
    }

    pub fn supports(&self, feature: ArchiveFeature) -> bool {
        match feature {
            ArchiveFeature::Create => true,
        }
    }
}

pub enum NyvoSource {
    File(&'static str),
}
