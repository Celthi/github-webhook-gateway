use super::user::User;
use crate::config_env;
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostComment<'a> {
    body: &'a str,
}
#[derive(Deserialize, Serialize)]
pub struct Issue {
    url: String,
    repository_url: String,
    comments_url: String,
    html_url: String,
    id: u64,
    number: u64,
    title: String,
    user: User,
    comments: u64,
    body: String,
}

impl Issue {
    pub fn get_number(&self) -> u64 {
        self.number
    }
}

pub async fn post_issue_comment(repo_name: &str, pr_number: u64, s: &str) -> Result<()> {
    let comment_url = format!(
        "https://github.com/api/v3/repos/{}/issues/{}/comments",
        repo_name, pr_number
    );

    let client = reqwest::Client::new();
    let data = &PostComment { body: s };

    match client
        .post(comment_url)
        .header(
            "Authorization",
            format!("token {}", config_env::get_github_token()),
        )
        .header("Accept", "application/vnd.github+json")
        .json(data)
        .send()
        .await
    {
        Ok(_) => {
            println!("{:?}", data);
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!(format!("post comment failed {}", e))),
    }
}
