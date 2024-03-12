use std::path::Path;

use git2::Repository;
use regex::Regex;

pub struct RepositoryManager {
    base_location: String,
}

impl RepositoryManager {
    pub fn new(base_location: &str) -> Self {
        RepositoryManager {
            base_location: base_location.to_string(),
        }
    }

    fn sanitize_name(name: &str) -> String {
        // replace whitespaces with hyphens and remove invalid characters
        let re = Regex::new(r"[^A-Za-z0-9-_.]").unwrap();
        re.replace_all(name, "-").into_owned()
    }

    pub async fn create_repository(&self, name: &str) -> Result<Repository, String> {
        let sanitized_name = Self::sanitize_name(name);
        let location = format!("{}/{}", self.base_location, sanitized_name);

        if Path::new(&location).exists() {
            return Err(format!("Repository already exists at: {}", location));
        }

        let repo: Repository = match Repository::init(&location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to init repository: {}", e)),
        };

        Ok(repo)
    }

    pub async fn clone_repository(&self, url: &str, name: &str) -> Result<Repository, String> {
        let sanitized_name = Self::sanitize_name(name);
        let location = format!("{}/{}", self.base_location, sanitized_name);

        if Path::new(&location).exists() {
            return Err(format!("Repository already exists at: {}", location));
        }

        let repo = match Repository::clone(url, &location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to clone repository: {}", e)),
        };

        Ok(repo)
    }
}
