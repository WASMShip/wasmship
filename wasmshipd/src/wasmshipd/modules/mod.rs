use std::path::{Path, PathBuf};

mod repositories;
mod types;
use repositories::Repositories;
use types::{ModuleError, ModuleResult};

pub struct Modules {
    path: String,
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
            Ok(Modules {
                path: path.to_string(),
                repositories,
            })
        } else {
            Err(ModuleError::NotFound(Some(base_path.to_path_buf())))
        }
    }

    pub fn get_module(&self, key: &str, tag: &str) -> Option<PathBuf> {
        if self.repositories.have_repository(key, tag) {
            let hash = self.repositories.get_hash(key, tag).unwrap();
            let path = Path::new(&self.path).join(&hash.hash).join("module.wasm");
            println!("{:?}", path);
            if path.exists() {
                return Some(path);
            }
        }
        None
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
