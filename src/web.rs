use crate::github::event::GithubEvent;

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
        .at("/backend_webhook", post(process_github_event_ep))
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
            "message": "Not backend_ comment.",
        }));
    }

    match GithubEvent::new(&req) {
        Ok(event) => match message::producer::produce_message_from(event) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("processing ocr event failed with: {}", e);
                return Json(serde_json::json! ({
                    "code": 0,
                    "message": format!("Error: {}", e)
                }));
            }
        },
        Err(e) => {
            eprintln!(
                "parsing eror for the github event: {} with error: {}",
                req, e
            );
            return Json(serde_json::json! ({
                "code": 400,
                "message": "Parsing github event failed comment.",
            }));
        }
    }

    Json(serde_json::json! ({
        "code": 0,
        "message": "Finish process github event.",
    }))
}
