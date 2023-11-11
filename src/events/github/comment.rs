use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Comment {
    url: String,
    html_url: String,
    issue_url: String,
    id: u64,
    user: User,
    body: String,
}

impl Comment {
    pub fn get_body(&self) -> &str {
        &self.body
    }
}
