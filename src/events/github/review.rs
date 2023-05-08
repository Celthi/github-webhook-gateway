use super::user::User;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Review {
    id: u64,
    user: User,
    body: String,
}
impl Review {
    pub fn get_number(&self) -> u64 {
        self.id
    }
    pub fn get_body(&self) -> &str {
        &self.body
    }
}
