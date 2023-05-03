#[cfg(not(target_os = "windows"))]
use super::kafka_dest;
use crate::msg::task::BackendTask;
use crate::msg::Message;
use crate::config_env;
use crate::github;
use crate::msg::queue;
use anyhow::Result;
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
                        kafka_dest::produce(
                            config_env::get_kafka_broker_list(),
                            config_env::get_kafka_time_spent_topic(),
                            &serde_json::to_vec(&tp).unwrap(),
                            tp.get_id(),
                        )
                        .await;
                    });
                } else {
                    if let (Some(repo), Some(pr)) =
                        (tp.get_repo().map(str::to_string), tp.get_pr_number())
                    {
                        tokio::spawn(async move {
                            let _ = github::post_issue_comment(&repo, pr, "Time And Task: not a valid time spent, please make sure your PR title to follow the guideline.").await;
                        });
                    }
                }
            }
            Message::BackendTask(backend_task) => {
                let task = backend_task.clone();
                tokio::spawn(async move {
                    kafka_dest::produce(
                        config_env::get_kafka_broker_list(),
                        config_env::get_kafka_topic(),
                        &serde_json::to_vec(&backend_task).unwrap(),
                        backend_task.PR,
                    )
                    .await;
                });
                if config_env::is_backend_api_enable() {
                    info!("Beginning sending task...");
                    tokio::spawn(async move {
                        if let Err(e) = sending_task(task).await {
                            eprintln!("sending_tak failed, error: {}", e);
                        }
                    });
                }
            }
        }
    }
}

async fn sending_task(backend_task: BackendTask) -> Result<()> {
    info!("sending job: {:?}", serde_json::to_string(&backend_task));
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}:{}/api/abavalidation",
            config_env::get_backend_host(),
            config_env::get_backend_port()
        ))
        .header("Authorization", config_env::get_backend_api_token().unwrap())
        .header("Accept", "application/json")
        .json(&backend_task)
        .send()
        .await;
    match res {
        Ok(body) => post_sending_task(body, &backend_task).await,
        Err(e) => {
            info!("Failed sending job {:?}", e);
            github::post_issue_comment(&backend_task.RepoName, backend_task.PR, &e.to_string())
                .await
        }
    }
}

async fn post_sending_task(body: reqwest::Response, backend_task: &BackendTask) -> Result<()> {
    info!("Succeed posting task {:?}", body);
    if body.status() != reqwest::StatusCode::OK {
        return Err(anyhow::anyhow!(format!(
            "Fail to send job with status code: {}.",
            body.status()
        )));
    }
    let result = body.json::<serde_json::Value>().await?;
    let code = result
        .get("code")
        .ok_or_else(|| anyhow::anyhow!("no code in it"))?;
    let code = code
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("code is not u64"))?;
    if code < 400 {
        return Ok(());
    }
    let error_message = format!(
        "\r\n{err}\r\nPlease read more details on doc: {doc}",
        err = result,
        doc = config_env::xt_doc_url()
    );

    github::post_issue_comment(&backend_task.RepoName, backend_task.PR, &error_message).await
}
