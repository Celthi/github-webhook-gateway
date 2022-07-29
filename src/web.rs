use crate::github;

use crate::message;
use anyhow::Result;
use poem::{
    handler, listener::TcpListener, middleware::Tracing, post, web::Json, EndpointExt, Route,
    Server,
};
use serde_json;
#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let router = Route::new();
    let app = router
        .at("/healthz", health_check)
        .at("/ocr_webhook", post(process_github_event_ep))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:31430"))
        .run(app)
        .await
}

#[handler(method = "get")]
async fn health_check() {}

#[handler]
fn process_github_event_ep(req: String) -> Json<serde_json::Value> {
    if !req.contains("KEYWORD") && !req.contains("KEYWORD2") {
        return Json(serde_json::json! ({
            "code": 0,
            "message": "Not interested comment.",
        }));
    }

    let event = match github::event::GithubEvent::new(&req) {
        Ok(event) => event,
        Err(e) => {
            eprintln!("Cannot process github message, error: {:?}", e);
            return Json(serde_json::json! ({
            "code": 0,
            "message": "Cannot process github message"}));
        }
    };
    if let Err(e) = message::producer::produce_message_from(&event) {
        eprintln!("Cannot process github message, error: {:?}", e);
        tokio::spawn(async move {
            let _ = github::issue::post_issue_comment(
                &event.get_repo_name(),
                event.get_pr_number(),
                &format!("{}", e),
            )
            .await;
        });
    }

    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish processing github event"}))
}
