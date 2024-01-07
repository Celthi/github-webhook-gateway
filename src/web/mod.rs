use anyhow::Result;
use axum::{routing::post, Router};
mod api;

#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    // build our application with a route
    let app = Router::new()
        .route("/ocr_webhook", post(api::github::process))
        .route("/rally_webhook", post(api::rally::process));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31430").await?;
    axum::serve(listener, app).await
}
