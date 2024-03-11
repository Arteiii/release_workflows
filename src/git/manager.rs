use git2::Repository;

pub struct RepositoryManager;

impl RepositoryManager {
    pub async fn create_repository(location: &str) -> Repository {
        let repo = match Repository::init(location) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        return repo;
    }
}
