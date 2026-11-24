use std::{
    ffi::OsStr,
    fmt::{self, Display, Formatter},
    ops::{Add, Sub},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FsPathComponent(String);

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct FsPath(Vec<FsPathComponent>);

impl FsPath {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn starts_with(&self, other: &FsPath) -> bool {
        self.0.starts_with(&other.0)
    }
}

impl From<&str> for FsPath {
    fn from(path: &str) -> Self {
        let mut components = vec![];
        for component in path.split('/') {
            if !component.is_empty() {
                for component in component.split('\\') {
                    if !component.is_empty() {
                        match component {
                            "." => continue,
                            ".." => {
                                components.pop();
                            }
                            _ => components.push(FsPathComponent(component.to_string())),
                        }
                    }
                }
            }
        }
        FsPath(components)
    }
}

impl From<&OsStr> for FsPath {
    fn from(path: &OsStr) -> Self {
        Self::from(path.to_string_lossy().as_ref())
    }
}

impl Display for FsPath {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(
            &self
                .0
                .iter()
                .map(|c| c.0.as_str())
                .collect::<Vec<_>>()
                .join("/"),
        )
    }
}

impl Add<&FsPath> for &FsPath {
    type Output = FsPath;

    fn add(self, rhs: &FsPath) -> Self::Output {
        let mut components = self.0.clone();
        components.extend(rhs.0.clone());
        FsPath(components)
    }
}

impl Sub<&FsPath> for &FsPath {
    type Output = Option<FsPath>;

    fn sub(self, rhs: &FsPath) -> Self::Output {
        if self.starts_with(&rhs) {
            let relative_path = FsPath(self.0[rhs.0.len()..].to_vec());
            Some(relative_path)
        } else {
            None
        }
    }
}
