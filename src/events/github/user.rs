use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct User {
    login: String,
    id: u64,
    url: String,
    repos_url: String,
    pub name: Option<String>,
}
