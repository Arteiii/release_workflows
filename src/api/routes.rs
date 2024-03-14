use std::process::Command;

use poem_openapi::{
    param,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, OpenApi,
};
use tracing::{debug, error, info};

use crate::build::{make, script};
use crate::git::manager::RepositoryManager as Repo;
use crate::util::file_system::FileSystem;
use crate::util::workflows::{workflows_exist, WorkflowScripts};

pub struct Api {
    repo_manager: Repo,
    file_system: FileSystem,
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

#[derive(ApiResponse)]
pub enum BuildRepo {
    /// Successfully -> Created
    #[oai(status = 201)]
    Ok(Json<String>),

    /// Server Errors -> Failed Build Response Body For Details
    #[oai(status = 500)]
    ServerError(Json<String>),
}

#[derive(ApiResponse)]
pub enum BuildScriptsResponse {
    /// Successfully -> Created
    #[oai(status = 201)]
    Ok(Json<WorkflowScripts>),

    /// Server Errors -> Failed Build Response Body For Details
    #[oai(status = 500)]
    ServerError(Json<String>),
}

#[derive(ApiResponse)]
pub enum SyncRepoResponse {
    /// Successfully -> Created
    #[oai(status = 201)]
    Ok(Json<String>),

    /// Server Errors -> Failed Build Response Body For Details
    #[oai(status = 500)]
    ServerError(Json<String>),
}

#[OpenApi]
impl Api {
    /// Constructs a new instance of `Api`.
    ///
    /// # Parameters
    ///
    /// * `repos_base_path`: Base path for repositories.
    ///
    /// # Returns
    ///
    /// A new instance of `Api`.
    pub fn new(repos_base_path: &str) -> Self {
        // Initialize RepoManager
        let repo_manager = Repo::new(repos_base_path);
        let file_system = FileSystem::new(repos_base_path);

        Api {
            repo_manager,
            file_system,
        }
    }

    /// Create a new Repository
    ///
    /// create new repo
    //
    //
    // removed for now
    // #[oai(path = "/repo/:name/create", method = "post")]
    // pub async fn create_repository(&self, name: param::Path<String>) -> CreateRepository {
    //     let name = name.to_string();
    //     match self.repo_manager.create_repository(&name).await {
    //         Ok(_) => {
    //             let msg = format!("{} Successfully Created", name);
    //             tracing::debug!(msg);
    //             CreateRepository::Ok(Json(msg))
    //         }
    //         Err(err) => {
    //             let err_msg = format!("{} failed to create ({})", name, err);
    //             tracing::error!(err_msg);
    //             CreateRepository::ServerError(Json(err_msg))
    //         }
    //     }
    // }

    /// Adds a new repository.
    ///
    /// # Parameters
    ///
    /// * `name`: Name of the repository.
    /// * `url`: URL of the repository.
    ///
    /// # Returns
    ///
    /// `AddRepository::Ok` if the repository is added successfully, otherwise `AddRepository::ServerError`.
    #[oai(path = "/repo/:name/add/:url", method = "post")]
    pub async fn add_repository(
        &self,
        name: param::Path<String>,
        url: param::Path<String>,
    ) -> AddRepository {
        debug!("adding repo {} from: {}", name.to_string(), url.to_string());
        match self.repo_manager.clone_repository(&url, &name).await {
            Ok(_) => {
                info!("repo is successfully cloned ({})", name.to_string());
                AddRepository::Ok
            }
            Err(err) => {
                error!("failed to clone the repo ({})", err);
                AddRepository::ServerError
            }
        }
    }

    /// Retrieves tags for a repository.
    ///
    /// # Parameters
    ///
    /// * `name`: Name of the repository.
    ///
    /// # Returns
    ///
    /// `GetTags::Ok` with repository tags if successful, otherwise `GetTags::NotFound`.
    #[oai(path = "/repo/:name/tags", method = "get")]
    pub async fn get_tags(&self, name: param::Path<String>) -> GetTags<Vec<String>> {
        debug!("requesting tags for ({})", name.to_string());
        match self.repo_manager.get_tags(&name).await {
            Ok(tags) => {
                info!("get tags successfully ({})", name.to_string());
                GetTags::Ok(Json(tags))
            }
            Err(e) => {
                error!("failed to get tags ({})", e);
                GetTags::NotFound
            }
        }
    }

    /// Builds a repository using the specified method.
    ///
    /// # Parameters
    ///
    /// * `name`: Name of the repository.
    ///
    /// * `method`: The build method to be used. Valid methods are "make", "script", "cargo", and "docker".
    ///
    /// # Folder Structure
    ///
    /// For the "make" method, a Makefile named "Makefile" must be present in the repository's `make/` directory.
    /// For the "script" method, the script must be named "build_script.sh" and placed in the repository's `script/` directory.
    ///
    /// Example folder structure:
    /// ```
    /// workflows/
    /// ├── make/
    /// │   └── Makefile
    /// └── script/
    /// │   └── build_script.sh
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// If the repository is built successfully, returns `GetTags::Ok` containing a message indicating success.
    /// If the provided method is invalid or if any error occurs during the build process, returns `GetTags::InternalServerError`
    /// with an appropriate error message.
    #[oai(path = "/repo/:name/build/:method", method = "put")]
    pub async fn build_repo(
        &self,
        name: param::Path<String>,
        method: param::Path<String>,
    ) -> BuildRepo {
        let repo_name = name.to_string();
        let method = method.to_string();

        let repo_path = &self.file_system.git_path(&repo_name);

        // Validate the method
        if !["make", "script", "cargo", "docker"].contains(&method.as_str()) {
            let err_msg = format!("Invalid build method: {}", method);
            error!(err_msg);
            return BuildRepo::ServerError(Json(err_msg));
        }

        // Check if the repository has the required build scripts
        let script_data = match workflows_exist(&repo_path) {
            Ok(script_data) => script_data,
            Err(err) => {
                let err_msg = format!("Failed to get build scripts: {}", err);
                error!(err_msg);
                return BuildRepo::ServerError(Json(err_msg));
            }
        };

        // Check if the specified method is available
        match method.as_str() {
            "make" if !script_data.has_makefile() => {
                let err_msg = "Makefile not found in the repository".to_string();
                error!(err_msg);
                return BuildRepo::ServerError(Json(err_msg));
            }
            "script" if !script_data.has_script() => {
                let err_msg = "Build script not found in the repository".to_string();
                error!(err_msg);
                return BuildRepo::ServerError(Json(err_msg));
            }
            _ => (),
        }

        // Execute the build process based on the method
        match method.as_str() {
            "cargo" => {
                let cargo_build_output = Command::new("cargo")
                    .args(&["build"])
                    .output()
                    .map_err(|e| format!("Failed to execute cargo build command: {}", e));

                if let Err(e) = cargo_build_output {
                    let err_msg = format!("Cargo build failed: {}", e);
                    error!(err_msg);
                    return BuildRepo::ServerError(Json(err_msg));
                }
            }
            "make" => {
                let make_build_output =
                    make::execute_makefile(&WorkflowScripts::get_makefile_path(&repo_path)).await;

                if let Err(e) = make_build_output {
                    let err_msg = format!("Make build failed: {}", e);
                    error!(err_msg);
                    return BuildRepo::ServerError(Json(err_msg));
                }
            }
            "script" => {
                let script_build_output = Command::new("sh")
                    .arg("./script/build_script.sh")
                    .output()
                    .map_err(|e| format!("Failed to execute build script: {}", e));

                if let Err(e) = script_build_output {
                    let err_msg = format!("Script build failed: {}", e);
                    error!(err_msg);
                    return BuildRepo::ServerError(Json(err_msg));
                }
            }
            // TODO: Add docker support
            // "docker" => {
            //     let docker_manager = docker::DockerManager::new("");
            //
            //     let docker_build_output = docker_manager.build_image(".", ".");
            //
            //     if let Err(e) = docker_build_output {
            //         let err_msg = format!("Docker build failed: {}", e);
            //         error!(err_msg);
            //         return BuildRepo::ServerError(Json(err_msg));
            //     }
            // }
            _ => (), // For unsupported methods, do nothing
        }

        let msg = format!("Build successful for latest commit ({})", repo_name);
        info!(msg);

        BuildRepo::Ok(Json(msg))
    }
    /// Retrieves the available build scripts for a repository.
    ///
    /// This endpoint is designed as support for the `/repo/:name/build/:method` endpoint.
    /// It returns a list of available scripts for building the repository.
    /// More details on scripts can be found in the description of the main endpoint.
    ///
    /// # Parameters
    ///
    /// * `name`: The name of the repository.
    ///
    /// # Returns
    ///
    /// If the operation succeeds, returns `BuildScriptsResponse::Ok` containing a JSON object
    /// representing the available build scripts. If an error occurs, returns `BuildScriptsResponse::ServerError`
    /// with an appropriate error message.
    ///
    #[oai(path = "/repo/:name/build", method = "get")]
    pub async fn get_build_scripts_for_repo(
        &self,
        name: param::Path<String>,
    ) -> BuildScriptsResponse {
        let repo_name = name.to_string();

        let git_path = self.file_system.git_path(&repo_name);

        // serialize the struct to json
        match workflows_exist(&git_path) {
            Ok(script_data) => {
                info!("get build script success ({})", name.to_string());
                BuildScriptsResponse::Ok(Json(script_data))
            }
            Err(err) => {
                let err_msg = format!("failed to get build scripts: {}", err);
                error!(err_msg);
                BuildScriptsResponse::ServerError(Json(err_msg))
            }
        }
    }

    /// Syncs a repository with its origin.
    ///
    /// This operation deletes the local repository and clones it again from the origin.
    /// As long as there are no local changes (as it should be), this operation is without risk.
    ///
    /// # Parameters
    ///
    /// * `name`: The name of the repository to sync.
    ///
    /// # Returns
    ///
    /// If the repository is successfully reset and synced with the origin, returns `SyncRepoResponse::Ok`
    /// with a success message. If an error occurs during the process, returns `SyncRepoResponse::ServerError`
    /// with an appropriate error message.
    ///
    #[oai(path = "/repo/:name/sync", method = "get")]
    pub async fn sync_repo_with_origin(&self, name: param::Path<String>) -> SyncRepoResponse {
        let repo_name = name.to_string();

        let git_path = self.file_system.git_path(&repo_name);
        debug!("syncing repo {} ", name.to_string());
        match self.repo_manager.sync_repo(&git_path).await {
            Ok(_) => {
                let msg = format!("Reset/synced repo successfully ({})", name.to_string());
                info!("{}", msg);
                SyncRepoResponse::Ok(Json(msg))
            }
            Err(err_msg) => {
                error!("{}", err_msg);
                SyncRepoResponse::ServerError(Json(err_msg))
            }
        }
    }
}
