use crate::config_env;
use crate::events::github;
use crate::kafka;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Default, ColonBuilder, Clone)]
struct OCRBody {
    #[cb(abbr = "BN")]
    BuildNo: String,
    #[cb(abbr = "SB")]
    ServerBranch: String,
    #[cb(abbr = "PS")]
    ProductStringBranch: String,
    #[cb(abbr = "BS")]
    BinSourceBranch: String,
    #[cb(abbr = "(XT ABAs|XT I'm running ABAs)")]
    ABAList: Vec<String>,
    ProjectIDs: Vec<String>,
    #[cb(abbr = "YB")]
    YatiBranch: String,
    #[cb(abbr = "MS")]
    MSTRSearch: String,
    TestType: String,
    MemoryMode: String,
    BuildType: String,
    #[cb(abbr = "(TC|TouchedComponents)")]
    TouchedComponents: Vec<String>,
    #[cb(abbr = "XY")]
    XYatiInfrastructure: String,
    Platform: Option<String>,
    Flag: Option<Vec<String>>,
    comment: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Task {
    APIToken: Option<String>,
    pub PR: u64,
    pub RepoName: String,
    Member: String,
    #[serde(flatten)]
    body: OCRBody,
}

impl Task {
    pub fn get_build_number(&self) -> &str {
        &self.body.BuildNo
    }
}

pub fn get_task_from_str(s: &str, repo: String, pr_number: u64, member: String) -> Result<Task> {
    let mut ocr_body = OCRBody::from_str(s);
    if ocr_body.BuildNo.is_empty() {
        bail!(format!(
            "\r\n Build number is **required**.\r\n Please read the {doc}",
            doc = config_env::xt_doc_url()
        ));
    }
    ocr_body.comment = Some(s.to_string());
    Ok(Task {
        APIToken: config_env::get_backend_api_token(),
        PR: pr_number,
        Member: member,
        body: ocr_body,
        RepoName: repo,
    })
}

pub async fn handle_task(task: Task) {
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
            if let Err(e) = sending_task(task2).await {
                eprintln!("sending_tak failed, error: {}", e);
            }
        });
    }
}

async fn sending_task(task: Task) -> Result<()> {
    info!("sending job: {:?}", serde_json::to_string(&task));
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}:{}/api/abavalidation",
            config_env::get_backend_host(),
            config_env::get_backend_port()
        ))
        .header(
            "Authorization",
            config_env::get_backend_api_token().unwrap(),
        )
        .header("Accept", "application/json")
        .json(&task)
        .send()
        .await;
    match res {
        Ok(body) => post_sending_task(body, &task).await,
        Err(e) => {
            info!("Failed sending job {:?}", e);
            github::post_issue_comment(&task.RepoName, task.PR, &e.to_string()).await
        }
    }
}

async fn post_sending_task(body: reqwest::Response, task: &Task) -> Result<()> {
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
    error!("{}{}{}", &task.RepoName, task.PR, &error_message);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_value() {
        use super::*;
        use crate::config_env;
        std::env::set_var("GITHUB_TOKEN", "value");
        std::env::set_var("KAFKA_BROKER_LIST", "value");
        std::env::set_var("KAFKA_TOPIC", "value");
        config_env::ensure_config();

        let s = r#"BN: 11.03.0064.0015
        name_abr"#;
        let t = get_task_from_str(s, "Tech/Server".to_string(), 12, "name_abr".to_string());

        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");

        let s = "BN: 11.03.0064.0015\r\n";
        let t = get_task_from_str(s, "Tech/Server".to_string(), 12, "name_abr".to_string());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
    }
    #[test]
    fn test_get_backend_task() {
        use crate::config_env;
        std::env::set_var("GITHUB_TOKEN", "value");
        std::env::set_var("KAFKA_BROKER_LIST", "value");
        std::env::set_var("KAFKA_TOPIC", "value");

        config_env::ensure_config();

        use super::get_task_from_str;
        let s = r#"BN: 11.03.0064.0015
        "#;
        let t = get_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
        let s = r#"BN: 11.03.0064.0015"#;
        let t = get_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
        let s = r#"BN: 11.03.0064.0015\r\nTC:Object"#;
        let t = get_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(
            t.unwrap().body.TouchedComponents,
            vec!["Object".to_string()]
        );
        let s = r#"BN: 11.03.0064.0015\r\nTouchedComponents:Object"#;
        let t = get_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(
            t.unwrap().body.TouchedComponents,
            vec!["Object".to_string()]
        );
        let s = r#"XT ABAs: T0, T15
        SB:xt-tt-11.3.1000.0125_installation_branch
        BN: 11.3.1000.0125
        TestType:Regression"#;
        let t = get_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.as_ref().unwrap().get_build_number(), "11.3.1000.0125");

        assert_eq!(t.as_ref().unwrap().body.TestType, "Regression".to_string());
        assert_eq!(
            t.as_ref().unwrap().body.ServerBranch,
            "xt-tt-11.3.1000.0125_installation_branch".to_string()
        );
        assert_eq!(t.as_ref().unwrap().body.Flag, None);
    }
}
