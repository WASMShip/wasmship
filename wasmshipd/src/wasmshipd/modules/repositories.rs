use super::types::{Hash, HashType, ModuleResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, io::Read};

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
        for (name, data) in repositories {
            let mut repo = Repository::new();
            for (tag, hash) in data {
                repo.add(&tag, &hash);
            }
            repos.inner.insert(name.clone(), repo);
        }
        Ok(repos)
    }

    pub fn have_repository(&self, key: &str, tag: &str) -> bool {
        if let Some(repo) = self.inner.get(key) {
            return repo.have_tag(tag);
        }
        false
    }

    pub fn get_hash(&self, key: &str, tag: &str) -> Option<Hash> {
        if let Some(repo) = self.inner.get(key) {
            if let Some(value) = repo.get_hash(tag) {
                let mut split = value.split(':');
                let hash_type: HashType = split.next().unwrap().into();
                let hash_value = split.next().unwrap();
                return Some(Hash {
                    hash_type,
                    hash: hash_value.to_string(),
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    inner: HashMap<String, String>,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, tag: &str, hash: &str) {
        self.inner.insert(tag.to_string(), hash.to_string());
    }

    pub fn have_tag(&self, tag: &str) -> bool {
        self.inner.contains_key(tag)
    }

    pub fn get_hash(&self, tag: &str) -> Option<String> {
        self.inner.get(tag).map(|s| s.to_string())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_load_repoistries() {
        let repo = Repositories::from_file("tests/modules/repositories.json").unwrap();
        assert_eq!(repo.inner.len(), 1);
        assert!(repo.have_repository("mymod", "latest"));
    }
}
