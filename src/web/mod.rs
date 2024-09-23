use anyhow::Result;
mod api;

use axum::{
    extract::MatchedPath,
    http::Request,
    routing::post,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    // build our application with a route
    let app = Router::new()
        .route("/ocr_webhook", post(api::github::process))
        .route("/rally_webhook", post(api::rally::process))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31430").await?;
    axum::serve(listener, app).await
}
