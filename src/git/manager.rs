use git2::Repository;

#[allow(dead_code)]
pub struct RepositoryManager;

impl RepositoryManager {
    #[allow(dead_code)]
    pub fn create_repository(location: &str) -> Repository {
        let repo = match Repository::init(location) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        return repo;
    }
}
