use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sender {
    login: String,
    id: u64,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    ldap_dn: String,
}
