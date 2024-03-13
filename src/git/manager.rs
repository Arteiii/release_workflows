use std::path::Path;

use git2::Repository;
use tokio::time;

use crate::util::file_system::FileSystem;

pub struct RepositoryManager {
    file_system: FileSystem,
}

pub struct TagInfo {
    pub name: String,
    pub target_commit_id: String,
    pub tagger_name: String,
    pub tagger_email: String,
    // TODO: tagger_when add timestamp of tag (formatting issues)
    pub message: String,
}

impl RepositoryManager {
    pub fn new(base_location: &str) -> Self {
        let file_system = FileSystem::new(&base_location);
        RepositoryManager { file_system }
    }

    // pasted prob not working
    // fn fast_forward(&self, path: &Path) -> Result<(), Error> {
    //     let repo = Repository::open(path)?;
    //
    //     repo.find_remote("origin")?
    //         .fetch(&[self.branch], None, None)?;
    //
    //     let fetch_head = repo.find_reference("FETCH_HEAD")?;
    //     let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    //     let analysis = repo.merge_analysis(&[&fetch_commit])?;
    //     if analysis.0.is_up_to_date() {
    //         Ok(())
    //     } else if analysis.0.is_fast_forward() {
    //         let refname = format!("refs/heads/{}", self.branch);
    //         let mut reference = repo.find_reference(&refname)?;
    //         reference.set_target(fetch_commit.id(), "Fast-Forward")?;
    //         repo.set_head(&refname)?;
    //         repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
    //     } else {
    //         Err(Error::from_str("Fast-forward only!"))
    //     }
    // }

    // that is sooo stupid but tbh I don!t know how else to "pull" / sync changes using git2
    async fn reset_repository_using_origin(location: &str) -> Result<Repository, String> {
        // open the repository
        let repo = match Repository::open(location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to open repository: {}", e)),
        };

        // get the remote url
        let remote_url = match repo.find_remote("origin") {
            Ok(remote) => match remote.url() {
                Some(url) => url.to_string(),
                None => return Err("Remote URL not found".to_string()),
            },
            Err(e) => return Err(format!("Failed to find remote: {}", e)),
        };

        // delete local repository
        match std::fs::remove_dir_all(location) {
            Ok(()) => (),
            Err(e) => return Err(format!("Failed to delete repository: {}", e)),
        };

        // clone again
        let repo = match Repository::clone(&remote_url, &location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to clone repository: {}", e)),
        };

        Ok(repo)
    }

    pub async fn create_repository(&self, name: &str) -> Result<Repository, String> {
        let location = self.file_system.git_path(&name);

        if Path::new(&location).exists() {
            return Err(format!("Repository already exists at: {}", location));
        }

        let repo: Repository = match Repository::init_bare(&location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to init repository: {}", e)),
        };

        Ok(repo)
    }

    pub async fn clone_repository(&self, url: &str, name: &str) -> Result<Repository, String> {
        let name = name.to_string();
        let location = self.file_system.git_path(&name);

        if Path::new(&location).exists() {
            return Err(format!("Repository already exists at: {}", location));
        }

        let repo = match Repository::clone(url, &location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to clone repository: {}", e)),
        };

        // schedule a periodic task to pull updates every hour
        tokio::spawn(async move {
            loop {
                // Reset the repository to the state of the remote
                match RepositoryManager::reset_repository_using_origin(&location).await {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Failed to reset repository ({})", e);
                    }
                };

                // Sleep for 1 hour
                time::sleep(time::Duration::from_secs(3600)).await;
            }
        });

        Ok(repo)
    }

    pub async fn get_tags(&self, name: &str) -> Result<Vec<String>, String> {
        let location = self.file_system.git_path(&name);

        let repo = match Repository::open(&location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to open repository: {}", e)),
        };

        let tag_names = match repo.tag_names(None) {
            Ok(tag_names) => tag_names,
            Err(e) => return Err(format!("Failed to retrieve tags: {}", e)),
        };

        let tag_infos: Vec<String> = tag_names
            .iter()
            .filter_map(|tag_name| tag_name.map(|name| name.to_string()))
            .collect();

        Ok(tag_infos)
    }

    pub async fn sync_repo(&self, path: &str) -> Result<(), String> {
        match RepositoryManager::reset_repository_using_origin(&path).await {
            Ok(_) => {
                let msg = format!("reset/synced repo at {}", path);
                tracing::info!("{}", msg);
                Ok(())
            }
            Err(e) => {
                let err_msg = format!("failed to reset repository ({})", e);
                tracing::error!("{}", err_msg);
                Err(err_msg)
            }
        }
    }
}
