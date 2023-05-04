use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::env;
use std::process;

#[derive(Debug)]
pub struct ConfigEnv {
    pub backend_host: Option<String>,
    pub backend_port: Option<String>,
    pub backend_api_token: Option<String>,
    pub github_token: String,
    pub kafka_broker_list: String,
    pub kafka_topic: String,
    pub time_spent_topic: String,
    pub xt_doc_url: String,
}

impl ConfigEnv {
    pub fn new() -> Result<ConfigEnv> {
        let github_token = env::var("GITHUB_TOKEN");
        if github_token.is_err() {
            return Err(anyhow!(
                "GITHUB_TOKEN is required, please provide it by env variable GITHUB_TOKEN"
            ));
        }

        let kafka_broker_list = env::var("KAFKA_BROKER_LIST");
        if kafka_broker_list.is_err() {
            return Err(anyhow!(
                "KAFKA_BROKER_LIST is required, please provide it by env variable KAFKA_BROKER_LIST like localhost:9092"
            ));
        }

        let kafka_topic = env::var("KAFKA_TOPIC");
        if kafka_topic.is_err() {
            return Err(anyhow!(
                "KAFKA_TOPIC is required, please provide it by env variable KAFKA_TOPIC"
            ));
        }
        let mut time_spent_topic = env::var("KAFKA_TP_TOPIC");
        if time_spent_topic.is_err() {
            time_spent_topic = Ok("time_spent".to_string());
        }
        let xt_doc_url = env::var("XT_DOC_URL").unwrap_or_else(|_| "".to_string());

        Ok(ConfigEnv {
            backend_host: env::var("BACKEND_HOST").ok(),
            backend_port: env::var("BACKEND_PORT").ok(),
            backend_api_token: env::var("BACKEND_API_TOKEN").ok(),
            github_token: github_token.unwrap(),
            kafka_broker_list: kafka_broker_list.unwrap(),
            kafka_topic: kafka_topic.unwrap(),
            time_spent_topic: time_spent_topic.unwrap(),
            xt_doc_url,
        })
    }
}

pub static CONFIG: OnceCell<ConfigEnv> = OnceCell::new();

pub fn is_backend_api_enable() -> bool {
    let config = CONFIG.get().expect("fail to get env variable");
    config.backend_host.is_some()
        && config.backend_port.is_some()
        && config.backend_api_token.is_none()
}
pub fn get_backend_host() -> &'static str {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_host
        .as_ref()
        .unwrap()
}
pub fn get_backend_port() -> &'static str {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_port
        .as_ref()
        .unwrap()
}
pub fn get_backend_api_token() -> Option<String> {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_api_token
        .to_owned()
}
pub fn get_github_token() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").github_token
}

pub fn get_kafka_broker_list() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .kafka_broker_list
}
pub fn get_kafka_time_spent_topic() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .time_spent_topic
}

pub fn get_kafka_topic() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").kafka_topic
}
pub fn xt_doc_url() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").xt_doc_url
}

pub fn ensure_config() {
    match ConfigEnv::new() {
        Ok(c) => {
            if let Err(e) = CONFIG.set(c) {
                eprintln!("creating config_env failed: {:?}", e);
            }
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
    if is_backend_api_enable() {
        println!("{}", get_backend_host());
        println!("{}", get_backend_port());
    }

    println!("{}", get_kafka_broker_list());
    println!("{}", get_kafka_topic());
    println!("{}", get_kafka_time_spent_topic());
}
