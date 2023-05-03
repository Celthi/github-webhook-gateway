
use anyhow::Result;
use poem::{
    handler, listener::TcpListener, middleware::Tracing, post, EndpointExt, Route,
    Server,
};

mod api;
#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let router = Route::new();
    let app = router
        .at("/healthz", health_check)
        .at("/ocr_webhook", post(api::github::process))
        .at("/rally_webhook", post(api::rally::process))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:31430"))
        .run(app)
        .await
}

#[handler(method = "get")]
async fn health_check() {}
