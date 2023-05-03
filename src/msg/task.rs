use crate::config_env;
use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

macro_rules! reg {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

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
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct BackendTask {
    APIToken: Option<String>,
    pub PR: u64,
    pub RepoName: String,
    Member: String,
    #[serde(flatten)]
    body: OCRBody,
}

impl BackendTask {
    pub fn get_build_number(&self) -> &str {
        &self.body.BuildNo
    }
}

pub fn get_backend_task_from_str(
    s: &str,
    repo: String,
    pr_number: u64,
    member: String,
) -> Result<BackendTask> {
    let ocr_body = OCRBody::from_str(s);
    if ocr_body.BuildNo.is_empty() {
        return Err(anyhow!(format!(
            "\r\n Build number is required.\r\n Please read the {doc}",
            doc = config_env::xt_doc_url()
        )));
    }

    Ok(BackendTask {
        APIToken: config_env::get_backend_api_token(),
        PR: pr_number,
        Member: member,
        body: ocr_body,
        RepoName: repo,
    })
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
        let t = get_backend_task_from_str(s, "Tech/Server".to_string(), 12, "name_abr".to_string());

        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");

        let s = "BN: 11.03.0064.0015\r\n";
        let t = get_backend_task_from_str(s, "Tech/Server".to_string(), 12, "name_abr".to_string());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
    }
    #[test]
    fn test_get_backend_task() {
        use crate::config_env;
        std::env::set_var("GITHUB_TOKEN", "value");
        std::env::set_var("KAFKA_BROKER_LIST", "value");
        std::env::set_var("KAFKA_TOPIC", "value");

        config_env::ensure_config();

        use super::get_backend_task_from_str;
        let s = r#"BN: 11.03.0064.0015
        "#;
        let t = get_backend_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
        let s = r#"BN: 11.03.0064.0015"#;
        let t = get_backend_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.unwrap().get_build_number(), "11.03.0064.0015");
        let s = r#"BN: 11.03.0064.0015\r\nTC:Object"#;
        let t = get_backend_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(
            t.unwrap().body.TouchedComponents,
            vec!["Object".to_string()]
        );
        let s = r#"BN: 11.03.0064.0015\r\nTouchedComponents:Object"#;
        let t = get_backend_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(
            t.unwrap().body.TouchedComponents,
            vec!["Object".to_string()]
        );
        let s = r#"XT ABAs: T0, T15
        SB:xt-tt-11.3.1000.0125_installation_branch
        BN: 11.3.1000.0125
        TestType:Regression"#;
        let t = get_backend_task_from_str(s, "ro".to_string(), 121, "mt".to_owned());
        assert_eq!(t.as_ref().unwrap().get_build_number(), "11.3.1000.0125");

        assert_eq!(t.as_ref().unwrap().body.TestType, "Regression".to_string());
        assert_eq!(
            t.as_ref().unwrap().body.ServerBranch,
            "xt-tt-11.3.1000.0125_installation_branch".to_string()
        );
        assert_eq!(t.as_ref().unwrap().body.Flag, None);
    }
}
