use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::env;
use std::process;
#[derive(Debug)]
pub struct ConfigEnv {
    pub backend_host: String,
    pub backend_port: String,
    pub backend_api_token: String,
    pub github_token: String,
}

impl ConfigEnv {
    pub fn new() -> Result<ConfigEnv> {
        let backend_host = env::var("BACK_END_HOST");
        if backend_host.is_err() {
            return Err(anyhow!(
                "Toil Host is required, please provide it by env variable BACK_END_HOST"
            ));
        }
        let backend_port = env::var("BACK_END_PORT");
        if backend_port.is_err() {
            return Err(anyhow!(
                "Toil port is required, please provide it by env variable BACK_END_PORT"
            ));
        }
        let backend_api_token = env::var("BACK_END_API_TOKEN");
        if backend_api_token.is_err() {
            return Err(anyhow!(
                "Toil port is required, please provide it by env variable BACK_END_API_TOKEN"
            ));
        }
        let github_token = env::var("GITHUB_TOKEN");
        if github_token.is_err() {
            return Err(anyhow!(
                "GITHUB_TOKEN is required, please provide it by env variable GITHUB_TOKEN"
            ));
        }
        Ok(ConfigEnv {
            backend_host: backend_host.unwrap(),
            backend_port: backend_port.unwrap(),
            backend_api_token: backend_api_token.unwrap(),
            github_token: github_token.unwrap(),
        })
    }
}

pub static CONFIG: OnceCell<ConfigEnv> = OnceCell::new();

pub fn get_backend_host() -> String {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_host
        .clone()
}
pub fn get_backend_port() -> String {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_port
        .clone()
}
pub fn get_backend_api_token() -> String {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .backend_api_token
        .clone()
}
pub fn get_github_token() -> String {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .github_token
        .clone()
}

pub fn ensure_config() {
    match ConfigEnv::new() {
        Ok(c) => {
            if let Err(e) = CONFIG.set(c) {
                eprintln!("reading env variable failed: {:?}", e);
            }
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
    println!("{}", get_backend_host());
    println!("{}", get_backend_port());
    println!("{}", get_backend_api_token());
}
