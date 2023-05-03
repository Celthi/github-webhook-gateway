use super::{comment::Comment};
use super::issue::Issue;
use super::pull_request::PullRequest;
use super::review::Review;
use super::repo::Repository;
use anyhow::Result;
use super::sender::Sender;
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
impl GithubEvent {
    pub fn get_code(&self) -> Option<&str> {
        if self.comment.is_some() {
            return Some(self.comment.as_ref().unwrap().get_body());
        }
        if self.review.is_some() {
            return Some(self.review.as_ref().unwrap().get_body())
        }
        None
    }
    pub fn new(payload: &str) -> Result<GithubEvent> {
        let res: GithubEvent = serde_json::from_str(payload)?;
        Ok(res)
    }
    pub fn get_repo_name(&self) -> Option<&str> {
        Some(self.repository.get_full_name())
    }
    pub fn get_pr_number(&self) -> Option<u64> {
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
    pub fn get_action(&self) -> &str {
        &self.action
    }
    pub fn get_user_name(&self) -> String {
        if let Some(name) = self.sender.get_sender_name() {
            name
        } else {
            "ocr_default".to_string()
        }
    }
    pub fn get_login_name(&self) -> &str {
        self.sender.get_login_name()
    }

    pub fn get_work_product(&self) -> Option<String> {
        if self.pull_request.is_some() {
            return self.pull_request.as_ref().unwrap().get_work_product();
        }
        if let Some(s) = self.issue.as_ref().unwrap().get_work_product() {
            return Some(s);
        }
        
        None
    }
}
