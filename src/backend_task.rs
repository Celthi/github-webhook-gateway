use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Builder, Default)]
#[builder(default)]
pub struct Task {
    pub RepoName: String,
    pub PR: u64,
}

fn get_value_by_field(f: &str, s: &str) -> Option<String> {
    let re = Regex::new(&format!(r"{f}:\s*(?P<v>.+?)((\r?\n)|$)")).unwrap();
    if let Some(m) = re.captures(s) {
        m.name("v").map(|s| s.as_str().trim().to_owned())
    } else {
        None
    }
}

pub fn get_backend_task_from_str(
    s: &str,
    repo: &str,
    pr_number: u64,
    member: String,
) -> Result<Task> {
    let mut t = TaskBuilder::default();
    t.RepoName(repo.to_owned());
    t.PR(pr_number);
    Ok(t.build().unwrap())
}
