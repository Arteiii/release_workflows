use poem::Result;
use poem_openapi::payload::PlainText;
use poem_openapi::{param, OpenApi, Tags};

use crate::git::manager::RepositoryManager as Repo;
use tokio::fs;

fn is_valid_folder_name(name: &str) -> bool {
    // Check for invalid characters in Windows file system
    let invalid_chars_windows = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    if cfg!(windows) && name.chars().any(|c| invalid_chars_windows.contains(&c)) {
        return false;
    }

    // Check for invalid characters in Unix file system
    let invalid_chars_unix = ['/'];
    if cfg!(unix) && name.chars().any(|c| invalid_chars_unix.contains(&c)) {
        return false;
    }

    // Additional checks can be added based on specific requirements

    // If no invalid characters were found, the name is valid
    true
}

#[derive(Default)]
pub struct Api {}

#[derive(Tags)]
enum MyTags {
    V1,
}

#[OpenApi]
impl Api {
    /// Greet the customer
    ///
    /// # Example
    ///
    /// Call `/1234/hello` to get the response `"Hello 1234!"`.
    #[oai(path = "/hello/:name", method = "get", tag = "MyTags::V1")]
    pub async fn index(&self, name: param::Path<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("Hello {}!", name)),
            None => PlainText("Hello!".to_string()),
        }
    }

    /// Create new Repository
    #[oai(path = "/repository/:name", method = "post")]
    pub async fn add_repository(&self, name: param::Path<String>) -> Result<PlainText<String>> {
        let sanitized_name = name.to_string().replace(' ', "-");
        let base_path = "E:/RepoTests";
        let location = format!("{}/Repos/{}", base_path, sanitized_name);

        // check if the path already exists
        if std::path::Path::new(&location).exists() {
            return Ok(PlainText(format!(
                "Repository already exists: {}",
                sanitized_name
            )));
        }

        // check if the sanitized name is valid
        if is_valid_folder_name(&sanitized_name) {
            // create the repository
            let _repo = Repo::create_repository(&location).await;
            Ok(PlainText(format!("Created Repo: {}", location)))
        } else {
            Ok(PlainText(format!("Failed To Create: {}", sanitized_name)))
        }
    }
}
