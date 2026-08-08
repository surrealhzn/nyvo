use std::{path::PathBuf, rc::Rc};

use crate::Res;

mod config;
pub use config::Config;

pub enum Warning {
    DirectoryTraversal(String),
}

pub struct Environment {
    pub config: Config,
    pub warn_fn: Box<dyn Fn(Warning) + Send + Sync>,
}

impl Environment {
    pub fn new(warn_fn: Box<dyn Fn(Warning) + Send + Sync>) -> Res<Self> {
        Ok(Self {
            config: Config::default(),
            warn_fn,
        })
    }

    pub fn new_config(config: Config, warn_fn: Box<dyn Fn(Warning) + Send + Sync>) -> Res<Self> {
        Ok(Self { config, warn_fn })
    }

    pub fn new_custom_config(
        _path: PathBuf,
        warn_fn: Box<dyn Fn(Warning) + Send + Sync>,
    ) -> Res<Self> {
        Ok(Self {
            config: Config::default(),
            warn_fn,
        })
    }
}

pub type Env = Rc<Environment>;
