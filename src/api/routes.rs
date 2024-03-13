use poem_openapi::{
    param,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, OpenApi,
};

use crate::git::manager::RepositoryManager as Repo;

pub struct Api {
    repo_manager: Repo,
}

#[derive(ApiResponse)]
pub enum AddRepository {
    /// Successfully -> Created
    #[oai(status = 201)]
    Ok,

    /// Server Errors -> Internal Server Error
    #[oai(status = 500)]
    ServerError,
}

#[derive(ApiResponse)]
pub enum GetTags<T: ParseFromJSON + ToJSON> {
    /// Successfully -> OK
    #[oai(status = 200)]
    Ok(Json<T>),

    /// Client Error -> Not Found
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum CreateRepository {
    /// Successfully -> Created
    #[oai(status = 201)]
    Ok(Json<String>),

    /// Server Errors -> Failed Response Body For Details
    #[oai(status = 500)]
    ServerError(Json<String>),
}

#[OpenApi]
impl Api {
    pub fn new(repos_base_path: &str) -> Self {
        // Initialize RepoManager
        let repo_manager = Repo::new(repos_base_path);

        Api { repo_manager }
    }

    /// Create a new Repository
    ///
    /// create new repo
    #[oai(path = "/repo/:name/create", method = "post")]
    pub async fn create_repository(&self, name: param::Path<String>) -> CreateRepository {
        let name = name.to_string();
        match self.repo_manager.create_repository(&name).await {
            Ok(_) => {
                let msg = format!("{} Sucessfully Created", name);
                tracing::debug!(msg);
                CreateRepository::Ok(Json(msg))
            }
            Err(err) => {
                let err_msg = format!("{} failed to create ({})", name, err);
                tracing::error!(err_msg);
                CreateRepository::ServerError(Json(err_msg))
            }
        }
    }

    /// add a new Repository
    ///
    /// add new repo form url
    #[oai(path = "/repo/:name/add/:url", method = "post")]
    pub async fn add_repository(
        &self,
        name: param::Path<String>,
        url: param::Path<String>,
    ) -> AddRepository {
        tracing::debug!("adding repo {} from: {}", name.to_string(), url.to_string());
        match self.repo_manager.clone_repository(&url, &name).await {
            Ok(_) => {
                tracing::debug!("repo is successfully cloned ({})", name.to_string());
                AddRepository::Ok
            }
            Err(err) => {
                tracing::error!("failed to clone the repo ({})", err);
                AddRepository::ServerError
            }
        }
    }

    /// get repo tags
    #[oai(path = "/repo/:name/tags", method = "get")]
    pub async fn get_tags(&self, name: param::Path<String>) -> GetTags<Vec<String>> {
        tracing::debug!("requesting tags for ({})", name.to_string());
        match self.repo_manager.get_tags(&name).await {
            Ok(tags) => {
                tracing::debug!("get tags successfully ({})", name.to_string());
                GetTags::Ok(Json(tags))
            }
            Err(e) => {
                tracing::error!("failed to get tags ({})", e);
                GetTags::NotFound
            }
        }
    }
}
