use super::get_work_product;
use super::user::User;

use serde::{Deserialize, Serialize};

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
    body: Option<String>,
}

impl Issue {
    pub fn get_number(&self) -> u64 {
        self.number
    }
    pub fn get_work_product(&self) -> Option<String> {
        get_work_product(&self.title)
    }
}
