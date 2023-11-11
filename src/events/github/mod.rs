mod comment;
pub mod event;
pub mod handler;
pub mod issue;
mod owner;
mod pull_request;
mod repo;
mod review;
mod sender;
mod user;
use crate::config_env;
use crate::reg;
use anyhow::{anyhow, Result};
use reqwest;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::error;

use self::user::User;

pub fn get_work_product(s: &str) -> Option<String> {
    let pat = reg!(r"(?P<item_id>((DE)|(US))\d{4,8}(\s*,\s*((DE)|(US))\d{4,8})*)");
    let s_upper = s.to_uppercase();
    let m = pat.captures(&s_upper)?;
    m.name("item_id").map(|n| n.as_str().to_string())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostComment<'a> {
    body: &'a str,
}

pub async fn post_issue_comment(repo_name: &str, pr_number: u64, s: &str) -> Result<()> {
    let comment_url = format!(
        "https://github.com/api/v3/repos/{}/issues/{}/comments",
        repo_name, pr_number
    );

    let client = reqwest::Client::new();
    let data = &PostComment { body: s };

    let Err(e) = client
        .post(comment_url)
        .header(
            "Authorization",
            format!("token {}", config_env::get_github_token()),
        )
        .header("Accept", "application/vnd.github+json")
        .json(data)
        .send()
        .await
    else {
        println!("{:?}", data);
        return Ok(());
    };
    Err(anyhow::anyhow!(format!("post comment failed {}", e)))
}

pub async fn get_user_name(login: &str) -> Result<User> {
    let url = format!("https://api.github.com/users/{login}",);

    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", config_env::get_github_token()),
        )
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "chat-aba")
        .send()
        .await?;

    get_results::<user::User>(res).await
}

async fn get_results<T: DeserializeOwned>(resp: Response) -> Result<T> {
    let status = resp.status();
    let text = resp.text().await?;
    if status.is_success() {
        match serde_json::from_str::<T>(&text) {
            Ok(o) => Ok(o),
            Err(e) => {
                error!(
                    "cannot convert the Rally response to the object model: {:?}, text: {}",
                    e, text
                );
                Err(anyhow!("{:?}. {}", e, text))
            }
        }
    } else {
        error!("fetch response from Rally meet error: {}", text);
        Err(anyhow!(format!("Error while geting response from the Rally. Possible reason: 1. Rally server is down. 2 Your Rally API token is invalid. \r\n\r\n. Rally Response is: {}", text)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn extract_us() {
        assert_eq!(
            get_work_product("DE123455; hihkdfd"),
            Some("DE123455".to_string())
        );
        assert_eq!(
            get_work_product("DE123455: hihkdfd"),
            Some("DE123455".to_string())
        );
        assert_eq!(
            get_work_product("de123455: hihkdfd"),
            Some("DE123455".to_string())
        );
        assert_eq!(
            get_work_product("de123455: hihkdfd de1234556"),
            Some("DE123455".to_string())
        );
    }
}
