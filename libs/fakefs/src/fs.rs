use crate::{FsContext, FsPath};
use std::fs::read_dir;

#[derive(Clone)]
pub struct FsRoot;

impl FsContext for FsRoot {
    fn list(&self, path: &FsPath) -> Result<Vec<FsPath>, std::io::Error> {
        read_dir("/".to_string() + &path.to_string())?
            .map(|res| res.map(|e| e.file_name().as_os_str().into()))
            .collect()
    }
}
