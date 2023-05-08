use super::get_work_product;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PullRequest {
    url: String,
    number: u64,
    state: String,
    title: String,
}

impl PullRequest {
    pub fn get_number(&self) -> u64 {
        self.number
    }
    pub fn get_work_product(&self) -> Option<String> {
        get_work_product(&self.title)
    }
}
