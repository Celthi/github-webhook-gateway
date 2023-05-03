mod comment;
pub mod issue;
mod owner;
mod repo;
mod user;
mod pull_request;
mod review;
pub mod event;
mod sender;
pub mod handler;
use crate::reg;
use crate::config_env;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub fn get_work_product(s: &str) -> Option<String> {
  let pat = reg!(r"(?P<item_id>((DE)|(US))\d{4,8}(\s*,\s*((DE)|(US))\d{4,8})*)");
  let s_upper = s.to_uppercase();
  if let Some(m) = pat.captures(&s_upper) {
      if let Some(n) = m.name("item_id") {
          return Some(n.as_str().to_string());
      }
  }
  None
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


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn extract_us() {
        assert_eq!(get_work_product("DE123455; hihkdfd"), Some("DE123455".to_string()));
        assert_eq!(get_work_product("DE123455: hihkdfd"), Some("DE123455".to_string()));
        assert_eq!(get_work_product("de123455: hihkdfd"), Some("DE123455".to_string()));
        assert_eq!(get_work_product("de123455: hihkdfd de1234556"), Some("DE123455".to_string()));

    }
}
