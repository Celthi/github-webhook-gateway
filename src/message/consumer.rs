use crate::config_env;
use crate::github;
use crate::queue;
use crate::backend_task::Task;
use anyhow::Result;
use tokio;
use tracing::{info, error};

#[tokio::main]
pub async fn event_loop() {
    loop {
        let r = queue::get_receiver();
        let backend_task;
        {
            let guard = r.lock().expect("cannot get lock from receiver.");
            backend_task = guard
                .recv()
                .expect("get message failed from receiver.")
                .backend_task;
        }
        tokio::spawn(async move {
            if let Err(e) = sending_task(backend_task).await {
                error!("sending_tak failed, error: {}", e.to_string());
            }
        });
    }
}
async fn sending_task(backend_task: Task) -> Result<()> {
    info!("sending job: {:?}", serde_json::to_string(&backend_task));
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}:{}/api/abavalidation",
            config_env::get_backend_host(),
            config_env::get_backend_port()
        ))
        .header("Authorization", &config_env::get_backend_api_token())
        .header("Accept", "application/json")
        .json(&backend_task)
        .send()
        .await;
    match res {
        Ok(body) => post_sending_task(body, &backend_task).await,
        Err(e) => {
            info!("Failed sending job {:?}", e);
            github::issue::post_issue_comment(&backend_task.RepoName, backend_task.PR, &e.to_string())
                .await
        }
    }
}
async fn post_sending_task(body: reqwest::Response, backend_task: &Task) -> Result<()> {
    info!("Succeed posting task {:?}", body);
    if body.status() != reqwest::StatusCode::OK {
        return Err(anyhow::anyhow!(format!(
            "Fail to send job with status code: {}.",
            body.status()
        )));
    }
    let result = body.json::<serde_json::Value>().await?;
    let code = result.get("code").ok_or(anyhow::anyhow!("no code in it"))?;
    let code = code.as_u64().ok_or(anyhow::anyhow!("code is not u64"))?;
    if code < 400 {
        return Ok(());
    }


    github::issue::post_issue_comment(&backend_task.RepoName, backend_task.PR, "<error>").await
}
