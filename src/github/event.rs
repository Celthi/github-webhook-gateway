use super::comment::Comment;
use super::issue::Issue;
use super::repo::Repository;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GithubEvent {
    action: String,
    issue: Issue,
    comment: Comment,
    repository: Repository,
}
impl GithubEvent {
    pub fn get_comment(&self) -> String {
        self.comment.get_body()
    }
    pub fn new(payload: &str) -> Result<GithubEvent> {
        let res: GithubEvent = serde_json::from_str(payload)?;
        Ok(res)
    }
    pub fn get_repo_name(&self) -> String {
        self.repository.get_full_name()
    }
    pub fn get_pr_number(&self) -> u64 {
        self.issue.get_number()
    }
    pub fn get_action(&self) -> String {
        self.action.clone()
    }
    pub fn get_user_name(&self) -> String {
        if let Some(name) = self.comment.get_sender_name() {
            name
        } else {
            "backend_default".to_owned()
        }
    }
}
