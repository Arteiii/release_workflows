use poem_openapi::{param, payload::PlainText, ApiResponse, OpenApi, Tags};

use crate::git::manager::RepositoryManager as Repo;

pub struct Api {
    repo_manager: Repo,
}

#[derive(ApiResponse)]
pub enum AddRepository {
    /// return when successfully
    #[oai(status = 200)]
    Ok,
    /// return when not
    #[oai(status = 404)]
    NotFound,

    /// return when not
    #[oai(status = 500)]
    ServerError,
}

#[derive(Tags)]
enum MyTags {
    V1,
}

#[OpenApi]
impl Api {
    pub fn new(repos_base_path: &str) -> Self {
        let repo_manager = Repo::new(repos_base_path);
        Api { repo_manager }
    }

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

    /// add a new Repository
    ///
    /// adds a new entry to the list of periodically checked repos
    #[oai(path = "/repository/:name", method = "post")]
    pub async fn add_repository(&self, name: param::Path<String>) -> AddRepository {
        match self.repo_manager.create_repository(&name).await {
            Ok(_) => {
                tracing::debug!("repo is successfully created ({})", name.to_string());
                AddRepository::Ok
            }
            Err(err) => {
                tracing::error!("failed to create the repo ({})", err);
                AddRepository::ServerError
            }
        }
    }
}
