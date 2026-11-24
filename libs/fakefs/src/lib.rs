use crate::fs::FsRoot;
use std::io;

pub use path::FsPath;

mod fs;
mod mem;
mod path;

pub trait FsContext: Clone {
    fn list(&self, path: &FsPath) -> Result<Vec<FsPath>, io::Error>;

    fn here(&self) -> Result<Vec<FsPath>, io::Error> {
        self.list(&FsPath::new())
    }
}

#[derive(Clone)]
pub struct FakeFs<T: FsContext> {
    context: T,
    path: FsPath,
}

impl FakeFs<FsRoot> {
    pub fn root() -> Self {
        Self {
            context: FsRoot,
            path: FsPath::new(),
        }
    }
}

impl<T: FsContext> FsContext for FakeFs<T> {
    fn list(&self, path: &FsPath) -> Result<Vec<FsPath>, io::Error> {
        let full_path = &self.path + path;
        self.context.list(&full_path)
    }
}
