use crate::{FsContext, FsPath};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MemFs(HashMap<FsPath, Vec<u8>>);

impl FsContext for MemFs {
    fn list(&self, path: &FsPath) -> Result<Vec<FsPath>, std::io::Error> {
        let mut result = vec![];
        for key in self.0.keys() {
            if let Some(relative_path) = path - key {
                result.push(relative_path);
            }
        }
        Ok(result)
    }
}
