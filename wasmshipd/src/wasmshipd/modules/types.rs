use std::path::PathBuf;
pub type ModuleResult<T> = Result<T, ModuleError>;

#[derive(Debug)]
pub enum ModuleError {
    // Files related to one module are missing
    NotFound(Option<PathBuf>),
    // Files related to one module are invalid
    BrokenFile(Option<PathBuf>),
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
