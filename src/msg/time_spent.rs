use crate::config_env;
use crate::github;
use crate::kafka;
use crate::reg;
use rand;
use serde::{Deserialize, Serialize};
use tracing::info;

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
pub trait TimeSpentTrait {
    fn get_repo_name(&self) -> Option<&str>;
    fn get_pr_number(&self) -> Option<u64>;
    fn get_user_name(&self) -> String;
    fn get_work_product(&self) -> Option<String>;
    fn get_code(&self) -> Option<&str>;
    fn get_login_name(&self) -> &str;
}

impl TimeSpent {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn is_valid(&self) -> bool {
        self.wp_formatted_id.is_some()
    }
    pub fn get_repo_name(&self) -> Option<&str> {
        self.repo_name.as_deref()
    }
    pub fn get_pr_number(&self) -> Option<u64> {
        self.pr_number
    }
}

pub fn get_time_spent<T: TimeSpentTrait>(
    s: &str,
    event: &T,
    task_name: Option<String>,
) -> Option<TimeSpent> {
    let pat = reg!(r"(T|t)hanks\s(?P<t>(\d{1})|(\d\.\d{1,3}))!");
    let user = event.get_user_name();
    let k = rand::random::<u64>();
    let wp = event.get_work_product();
    pat.captures(s).and_then(|m| m.name("t")).and_then(|n| {
        wp.map(|wp| TimeSpent {
            user,
            login: event.get_login_name().to_string(),
            value: n.as_str().parse().unwrap(),
            id: k,
            wp_formatted_id: Some(wp),
            repo_name: event.get_repo_name().map(|s| s.to_string()),
            pr_number: event.get_pr_number(),
            task_name,
        })
    })
}

pub async fn handle_time_spent(tp: TimeSpent) {
    info!("Beginning sending time spent...");
    if tp.is_valid() {
        tokio::spawn(async move {
            kafka::produce(
                config_env::get_kafka_broker_list(),
                config_env::get_kafka_time_spent_topic(),
                &serde_json::to_vec(&tp).unwrap(),
                tp.get_id(),
            )
            .await;
        });
    } else if let (Some(repo), Some(pr)) =
        (tp.get_repo_name().map(str::to_string), tp.get_pr_number())
    {
        tokio::spawn(async move {
            let _ = github::post_issue_comment(&repo, pr, "Time And Task: not a valid time spent, please make sure your PR title to follow the guideline.").await;
        });
    }
}
