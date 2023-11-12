pub mod handler;
use crate::events::msg::time_spent::TimeSpentTrait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// doc https://rally1.rallydev.com/apps/pigeon/docs/webhooks
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
    pub fn get_user_name(&self) -> &str {
        &self.message.transaction.user.username
    }
    pub fn get_action(&self) -> &str{
        &self.message.action
    }
}

impl TimeSpentTrait for Event {
    fn get_repo_name(&self) -> Option<&str> {
        None
    }
    fn get_pr_number(&self) -> Option<u64> {
        None
    }
    fn get_code(&self) -> Option<&str> {
        self.message.state.text.value.as_deref()
    }
    fn get_user_name(&self) -> String {
        self.message.transaction.user.username.clone()
    }
    fn get_work_product(&self) -> Option<String> {
        self.message
            .state
            .artifact
            .value
            .as_ref()
            .map(|a| a.formatted_id.clone())
    }
    fn get_login_name(&self) -> &str {
        &self.message.transaction.user.username
    }
}

#[derive(Serialize, Deserialize)]
struct Message {
    object_id: String,
    object_type: String,
    state: State,
    transaction: Transaction,
    action: String,
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
    value: Option<String>,
    name: String,
}
