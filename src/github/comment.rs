use super::user::User;
use regex::Regex;
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
    pub fn get_body(&self) -> String {
        self.body.clone()
    }
    pub fn get_sender_name(&self) -> Option<String> {
        let reg = Regex::new(r"CN=(?P<name>[^=]+),OU=").unwrap();
        if let Some(m) = reg.captures(&self.user.ldap_dn) {
            if let Some(n) = m.name("name") {
                return Some(n.as_str().replace('\\', ""));
            }
        }
        None
    }
}
