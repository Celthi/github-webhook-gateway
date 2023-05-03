use super::comment::Comment;
use super::issue::Issue;
use super::pull_request::PullRequest;
use super::repo::Repository;
use super::review::Review;
use super::sender::Sender;
use crate::events::msg::time_spent::TimeSpentTrait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct GithubEvent {
    action: String,
    issue: Option<Issue>,
    review: Option<Review>,
    comment: Option<Comment>,
    repository: Repository,
    sender: Sender,
    pull_request: Option<PullRequest>,
}
impl TimeSpentTrait for GithubEvent {
    fn get_repo_name(&self) -> Option<&str> {
        Some(self.repository.get_full_name())
    }
    fn get_pr_number(&self) -> Option<u64> {
        if self.pull_request.is_some() {
            return self.pull_request.as_ref().map(|p| p.get_number());
        }
        if self.issue.is_some() {
            return self.issue.as_ref().map(|i| i.get_number());
        }
        if self.review.is_some() {
            return self.review.as_ref().map(|r| r.get_number());
        }
        None
    }

    fn get_user_name(&self) -> String {
        self.sender
            .get_sender_name()
            .unwrap_or_else(|| "ocr_default".to_string())
    }

    fn get_work_product(&self) -> Option<String> {
        if self.pull_request.is_some() {
            return self
                .pull_request
                .as_ref()
                .and_then(|s| s.get_work_product());
        }
        self.issue.as_ref().and_then(|i| i.get_work_product())
    }

    fn get_code(&self) -> Option<&str> {
        if self.comment.is_some() {
            return Some(self.comment.as_ref().unwrap().get_body());
        }
        if self.review.is_some() {
            return Some(self.review.as_ref().unwrap().get_body());
        }
        None
    }

    fn get_login_name(&self) -> &str {
        self.sender.get_login_name()
    }
}
impl GithubEvent {
    pub fn new(payload: &str) -> Result<GithubEvent> {
        let res: GithubEvent = serde_json::from_str(payload)?;
        Ok(res)
    }

    pub fn get_action(&self) -> &str {
        &self.action
    }
}
