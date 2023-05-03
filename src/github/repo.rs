use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repository {
    id: u64,
    name: String,
    full_name: String,
    fork: bool,
    url: String,
    commits_url: String,
    issue_comment_url: String,
    pulls_url: String,
    git_url: String,
}

impl Repository {
    pub fn get_full_name(&self) -> &str {
        &self.full_name
    }
}
