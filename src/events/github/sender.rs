use crate::reg;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sender {
    login: String,
    id: u64,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    ldap_dn: Option<String>,
}

impl Sender {
    pub fn get_sender_name(&self) -> Option<String> {
        let reg = reg!(r"CN=(?P<name>[^=]+),OU=");
        if self.ldap_dn.is_none() {
            return Some(self.login.to_string());
        }
        if let Some(m) = reg.captures(self.ldap_dn.as_ref().unwrap()) {
            if let Some(n) = m.name("name") {
                return Some(n.as_str().replace('\\', ""));
            }
        }
        None
    }
    pub fn get_login_name(&self) -> &str {
        &self.login
    }
}
