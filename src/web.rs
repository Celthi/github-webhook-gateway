use crate::constants;
use crate::github;
use crate::msg;
use crate::rally;
use anyhow::Result;
use poem::{
    handler, listener::TcpListener, middleware::Tracing, post, web::Json, EndpointExt, Route,
    Server,
};
use serde_json;
use tracing::error;

#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let router = Route::new();
    let app = router
        .at("/healthz", health_check)
        .at("/ocr_webhook", post(process_github_event_ep))
        .at("/rally_webhook", post(process_rally_event_ep))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:31430"))
        .run(app)
        .await
}

#[handler(method = "get")]
async fn health_check() {}

#[handler]
fn process_rally_event_ep(req: String) -> Json<serde_json::Value> {
    if constants::contains_rally_pattern(&req) {
        match rally::Event::new(&req) {
            Ok(e) => {
                if let Err(e) = msg::producer::produce_msg_from(&e) {
                    error!("Cannot process rally message, error: {}{:?}", req, e);
                    return Json(serde_json::json! ({
                    "code": 0,
                    "message": "Finish processing github event"}));
                }
            }
            Err(e) => {
                error!("Cannot process rally message, error: {}{:?}", req, e);
            }
        }
    }
    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing rally event"}))
}
#[handler]
fn process_github_event_ep(req: String) -> Json<serde_json::Value> {
    if !constants::contain_keywords(&req) {
        return Json(serde_json::json! ({
            "code": 0,
            "message": "Not interested.",
        }));
    }

    let event = match github::event::GithubEvent::new(&req) {
        Ok(event) => event,
        Err(e) => {
            error!(
                "Cannot process github message, error: {:?}\n. payload is:\n{}",
                e, req
            );
            return Json(serde_json::json! ({
            "code": 0,
            "message": "Cannot process github message"}));
        }
    };
    if let Err(e) = msg::producer::produce_message_from(&event) {
        error!("Cannot process github message, error: {:?}", e);
        if let (Some(repo), Some(pr)) = (
            event.get_repo_name().map(str::to_string),
            event.get_pr_number(),
        ) {
            tokio::spawn(async move {
                let _ = github::post_issue_comment(&repo, pr, &format!("{}", e)).await;
            });
        }
    }

    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing github event"}))
}
