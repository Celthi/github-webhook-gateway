use crate::config_env;
use crate::constants;
use crate::backend_task::Task;
use anyhow::Result;
use serde_json;
pub async fn post_issue_comment(t: &Task, v: serde_json::Value) -> Result<()> {
    let comment_url = format!(
        constants::GITHUB_ISSUE_COMMENT_URL,
        t.RepoName, t.PR
    );
    let client = reqwest::Client::new();
    match client
        .post(comment_url)
        .header(
            "Authorization",
            format!("token {}", config_env::get_github_token()),
        )
        .header("Accept", "application/vnd.github+json")
        .json(&v)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(format!("post comment failed {}", e))),
    }
}
