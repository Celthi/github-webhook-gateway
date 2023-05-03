use anyhow::Result;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Event {
    message: Message,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    user: User,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
}
impl Event {
    pub fn new(payload: &str) -> Result<Self> {
        let res: Self = serde_json::from_str(payload)?;
        Ok(res)
    }
    pub fn get_code(&self) -> Option<&str> {
        Some(&self.message.state.text.value)
    }
    pub fn get_user_name(&self) -> &str {
        &self.message.transaction.user.username
    }
    pub fn get_work_product(&self) -> Option<&str> {
        self.message
            .state
            .artifact
            .value
            .as_ref()
            .map(|a| &*a.formatted_id)
    }
}
#[derive(Serialize, Deserialize)]
struct Message {
    object_id: String,
    object_type: String,
    state: State,
    transaction: Transaction,
}
#[derive(Serialize, Deserialize)]
struct State {
    #[serde(rename = "26803fda-2e78-4d9f-931d-84b8261d6f7b")]
    text: Text,
    #[serde(rename = "e43d0c61-7225-4e59-983f-ea05c2c6274d")]
    artifact: Artifact,
}

#[derive(Serialize, Deserialize)]
struct Artifact {
    value: Option<WorkProduct>,
}

#[derive(Serialize, Deserialize)]
struct WorkProduct {
    formatted_id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
struct Text {
    value: String,
    name: String,
}
