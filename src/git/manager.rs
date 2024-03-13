use std::path::Path;

use git2::Repository;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::time;

pub struct RepositoryManager {
    base_location: String,
}

#[derive(Serialize, Deserialize)]
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
        RepositoryManager {
            base_location: base_location.to_string(),
        }
    }

    fn git_path(base_location: &str, name: &str) -> String {
        // replace whitespaces with hyphens and remove invalid characters
        let re = Regex::new(r"[^A-Za-z0-9-_.]").unwrap();
        let sanitized_name = re.replace_all(name, "-").into_owned();

        format!("{}/{}.git", base_location, sanitized_name)
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

    pub async fn create_repository(&self, name: &str) -> Result<Repository, String> {
        let location = Self::git_path(&self.base_location, &name);

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
        let location = Self::git_path(&self.base_location, &name);

        if Path::new(&location).exists() {
            return Err(format!("Repository already exists at: {}", location));
        }

        let repo = match Repository::clone(url, &location) {
            Ok(repo) => repo,
            Err(e) => return Err(format!("Failed to clone repository: {}", e)),
        };

        // Schedule a periodic task to pull updates every hour
        tokio::spawn(async move {
            loop {
                // Fetch updates from the remote repository
                let mut remote = match repo.find_remote("origin") {
                    Ok(remote) => remote,
                    Err(e) => {
                        eprintln!("Failed to find remote: {}", e);
                        time::sleep(time::Duration::from_secs(3600)).await;
                        continue;
                    }
                };
                if let Err(e) = remote.fetch(&[], None, None) {
                    eprintln!("Failed to fetch updates from remote: {}", e);
                }

                // Pull updates into the local repository
                if let Err(e) = repo.pull(&[], None, None) {
                    eprintln!("Failed to pull updates into local repository: {}", e);
                }

                // Sleep for an hour before fetching updates again
                time::sleep(time::Duration::from_secs(3600)).await;
            }
        });

        Ok(repo)
    }

    pub async fn get_tags(&self, name: &str) -> Result<Vec<String>, String> {
        let location = Self::git_path(&self.base_location, &name);

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
}
