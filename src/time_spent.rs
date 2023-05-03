use crate::github::event::GithubEvent;
use crate::{event, reg};
use rand;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TimeSpent {
    user: String,
    login: String,
    value: f32,
    id: u64,
    wp_formatted_id: Option<String>,
    repo_name: Option<String>,
    pr_number: Option<u64>,
    task_name: Option<String>,
}

impl TimeSpent {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn is_valid(&self) -> bool {
        self.wp_formatted_id.is_some()
    }
    pub fn get_repo(&self) -> Option<&str> {
        self.repo_name.as_deref()
    }
    pub fn get_pr_number(&self) -> Option<u64> {
        self.pr_number
    }
}

pub fn get_time_spent_from_str(s: &str, event: &GithubEvent) -> Option<TimeSpent> {
    let pat = reg!(r"(T|t)hanks\s(?P<t>(\d{1})|(\d\.\d{1,3}))!");
    let user = event.get_user_name();
    let k = rand::random::<u64>();
    let wp = event.get_work_product();
    pat.captures(s)
        .and_then(|m| m.name("t"))
        .map(|n| TimeSpent {
            user,
            login: event.get_login_name().to_string(),
            value: n.as_str().parse().unwrap(),
            id: k,
            wp_formatted_id: wp,
            repo_name: event.get_repo_name().map(|s| s.to_string()),
            pr_number: event.get_pr_number(),
            task_name: None,
        })
}

pub fn get_time_spent_from_rally_str(s: &str, event: &event::rally::Event) -> Option<TimeSpent> {
    let pat = reg!(r"(T|t)hanks\s(?P<t>(\d{1})|(\d\.\d{1,3}))!");
    let user = event.get_user_name();
    let k = rand::random::<u64>();
    let wp = event.get_work_product();
    pat.captures(s).and_then(|m| m.name("t")).and_then(|n| {
        let user_name = event.get_user_name();
        wp.map(|wp| TimeSpent {
            user: user.to_string(),
            login: user_name.to_string(),
            value: n.as_str().parse().unwrap(),
            id: k,
            wp_formatted_id: Some(wp.to_string()),
            repo_name: None,
            pr_number: None,
            task_name: Some("Review and Suggestion.".to_string()),
        })
    })
}
