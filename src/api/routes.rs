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
    //             let msg = format!("{} Sucessfully Created", name);
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

        let git_path = &self.file_system.git_path(&repo_name);

        if !self.file_system.release_workflow_exists(&git_path) {
            let msg = format!("The release_workflow folder does not exist ({})", &git_path);
            error!(msg);
            return BuildRepo::ServerError(Json(msg.to_string()));
        }

        // validate the method
        if !["make", "script", "cargo", "docker"].contains(&method.as_str()) {
            let err_msg = format!("Invalid build method: {}", method);
            error!(err_msg);

            return BuildRepo::ServerError(Json(err_msg));
        }

        let (file_name, content) = match method.as_str() {
            "make" => (
                format!("{}.Makefile", &repo_name),
                make::generate_makefile(&repo_name).await,
            ),
            "script" => (
                format!("{}.sh", &repo_name),
                script::generate_script(&repo_name).await,
            ),
            _ => (String::new(), String::new()), // for cargo/docker, no file or content is needed
        };

        // write file to disk if needed
        // if !file_name.is_empty() && !content.is_empty() {
        //     if let Err(e) = write_to_file(&file_name, &content).wait {
        //         error!("Failed to write file: {}", e);
        //         BuildRepo::ServerError(Json(e));
        //     }
        // }

        // execute build process based on the method
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
                let make_build_output = Command::new("make")
                    .output()
                    .map_err(|e| format!("Failed to execute make command: {}", e));

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

            // TODO: add docker support
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
}
