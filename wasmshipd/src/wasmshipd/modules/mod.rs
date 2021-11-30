use std::path::Path;
mod repositories;
mod types;
pub use repositories::Module;
use repositories::Repositories;
use types::{ModuleError, ModuleResult};

pub struct Modules {
    repositories: Repositories,
}

impl Modules {
    pub fn load(path: &str) -> ModuleResult<Self> {
        let base_path = Path::new(path);
        if let Some(repo_index_path) = base_path.join("repositories.json").to_str() {
            let repositories = match Repositories::from_file(repo_index_path) {
                Ok(repositories) => repositories,
                Err(err) => return Err(err),
            };
            Ok(Modules { repositories })
        } else {
            Err(ModuleError::NotFound(Some(base_path.to_path_buf())))
        }
    }

    pub fn get_module(&self, key: &str, tag: &str) -> Option<Module> {
        self.repositories.get_module(key, tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_repoistries() {
        let modules = Modules::load("tests/modules").unwrap();
        assert!(modules.get_module("mymod", "latest").is_some());
    }
}
