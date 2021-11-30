use super::types::{ModuleError, ModuleResult};
use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

/// Structure for holding all modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repositories {
    pub inner: HashMap<String, Repository>,
}

impl Repositories {
    pub fn new() -> Self {
        Repositories {
            inner: HashMap::new(),
        }
    }

    pub fn from_file(path: &str) -> ModuleResult<Self> {
        let mut file = File::open(path)?;
        let mut file_content: String = String::new();
        file.read_to_string(&mut file_content)?;
        let content_map: HashMap<String, HashMap<String, HashMap<String, String>>> =
            serde_json::from_str(&file_content)?;
        let repositories = content_map.get("repositories").unwrap().clone();
        let mut repos = Repositories::new();
        for (key, data) in repositories {
            if let Some(repo_path) = Path::new(path).parent() {
                let mut repo = Repository::new(repo_path.to_path_buf());
                for (tag, hash) in data {
                    // We don't handle wrong module currently
                    let _ = repo.add(&tag, &hash);
                }
                if repo.len() > 0 {
                    repos.inner.insert(key.clone(), repo);
                }
            }
        }
        Ok(repos)
    }

    pub fn get_module(&self, key: &str, tag: &str) -> Option<Module> {
        if let Some(repo) = self.inner.get(key) {
            return repo.get_module(tag);
        }
        None
    }
}

/// Structure for holding modules with the same key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    path: PathBuf,
    inner: HashMap<String, Module>,
}

impl Repository {
    pub fn new(path: PathBuf) -> Self {
        Repository {
            path,
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, tag: &str, hash: &str) -> ModuleResult<()> {
        match Module::new(self.path.clone(), hash.to_string()) {
            Ok(module) => {
                self.inner.insert(tag.to_string(), module);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_module(&self, tag: &str) -> Option<Module> {
        self.inner.get(tag).cloned()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub path: PathBuf,
    pub hash: String,
    pub main: String,
    pub entry: Option<String>,
    pub link: Vec<String>,
}

// For serde_json
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleConfig {
    pub main: String,
    pub entry: Option<String>,
    pub link: Vec<String>,
}

impl Module {
    /// Load a module and validate it
    pub fn new(path: PathBuf, hash: String) -> ModuleResult<Module> {
        let path = path.join(hash.clone());
        let config_path = path.join("module.json");
        if !config_path.exists() {
            return Err(ModuleError::NotFound(Some(config_path)));
        }
        let mut file = File::open(config_path)?;
        let mut file_content: String = String::new();
        file.read_to_string(&mut file_content)?;
        let module_config: ModuleConfig = serde_json::from_str(&file_content)?;
        let module = Module {
            path,
            hash,
            main: module_config.main,
            entry: module_config.entry,
            link: module_config.link,
        };
        match module.validate() {
            Ok(_) => Ok(module),
            Err(e) => Err(e),
        }
    }

    /// Validate the module file
    ///
    /// Do following checks:
    /// - Check if the module file exists
    /// - Check if hash is correct
    ///
    /// If one check failed, the module won't be loaded.
    fn validate(&self) -> ModuleResult<()> {
        // Check if main module exists
        let module_path = self.path.join(&self.main);
        if !module_path.exists() {
            return Err(ModuleError::NotFound(Some(module_path)));
        }
        // Then check hash
        let module_file = File::open(module_path.clone())?;
        let reader = BufReader::new(module_file);
        let digest = sha256_digest(reader)?;
        if !self.hash.eq(&HEXLOWER.encode(digest.as_ref())) {
            return Err(ModuleError::BrokenFile(Some(module_path)));
        }
        // TODO: Check if linked modules exists
        Ok(())
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> std::io::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_load_repoistries() {
        let repo = Repositories::from_file("tests/modules/repositories.json").unwrap();
        assert_eq!(repo.inner.len(), 1);
        assert!(repo.get_module("mymod", "latest").is_some());
    }
}
