use std::path::PathBuf;
pub type ModuleResult<T> = Result<T, ModuleError>;

#[derive(Debug)]
pub enum ModuleError {
    NotFound(Option<PathBuf>),
    Io(std::io::Error),
}

impl From<std::io::Error> for ModuleError {
    fn from(err: std::io::Error) -> Self {
        ModuleError::Io(err)
    }
}

impl From<serde_json::Error> for ModuleError {
    fn from(err: serde_json::Error) -> Self {
        ModuleError::Io(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

pub enum HashType {
    Sha256,
}

impl From<&str> for HashType {
    fn from(_: &str) -> Self {
        // Currently only sha256 is supported
        HashType::Sha256
    }
}

pub struct Hash {
    pub hash: String,
    pub hash_type: HashType,
}