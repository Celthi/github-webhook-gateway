use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repository {
    id: u64,
    name: String,
    full_name: String,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    branches_url: String,
    commits_url: String,
    issue_comment_url: String,
    compare_url: String,
    merges_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    homepage: String,
    default_branch: String,
}

impl Repository {
    pub fn get_full_name(&self) -> String {
        self.full_name.clone()
    }
}
