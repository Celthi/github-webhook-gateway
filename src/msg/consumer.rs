use crate::kafka;
use crate::msg::Message;
use crate::config_env;
use crate::github;
use crate::msg::queue;
use crate::msg::task;
use tokio;
use tracing::info;

#[tokio::main]
pub async fn event_loop() {
    loop {
        let r = queue::get_receiver();
        let msg;
        {
            let guard = r.lock().expect("cannot get lock from receiver.");
            msg = guard.recv().expect("get message failed from receiver.");
        }
        match msg {
            Message::TimeSpent(tp) => {
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
                    (tp.get_repo().map(str::to_string), tp.get_pr_number())
                {
                    tokio::spawn(async move {
                        let _ = github::post_issue_comment(&repo, pr, "Time And Task: not a valid time spent, please make sure your PR title to follow the guideline.").await;
                    });
                }
            }
            Message::Task(task) => {
                let task2 = task.clone();
                tokio::spawn(async move {
                    kafka::produce(
                        config_env::get_kafka_broker_list(),
                        config_env::get_kafka_topic(),
                        &serde_json::to_vec(&task).unwrap(),
                        task.PR,
                    )
                    .await;
                });
                if config_env::is_backend_api_enable() {
                    info!("Beginning sending task...");
                    tokio::spawn(async move {
                        if let Err(e) = task::sending_task(task2).await {
                            eprintln!("sending_tak failed, error: {}", e);
                        }
                    });
                }
            }
        }
    }
}
