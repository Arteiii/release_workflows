use std::fs;

use regex::Regex;

pub struct FileSystem {
    pub base_location: String,
}

impl FileSystem {
    pub fn new(base_location: &str) -> Self {
        FileSystem {
            base_location: base_location.to_string(),
        }
    }

    pub fn git_path(&self, name: &str) -> String {
        // replace whitespaces with hyphens and remove invalid characters
        let re = Regex::new(r"[^A-Za-z0-9-_.]").unwrap();
        let sanitized_name = re.replace_all(name, "-").into_owned();

        format!("{}/{}", self.base_location, sanitized_name)
    }
}
