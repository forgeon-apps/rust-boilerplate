use axum::{
    http::header,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};

use crate::{logger, models, routes};

async fn v1_index() -> impl IntoResponse {
    Json(json!({
        "name": "rust-boilerplate-api",
        "version": "1.0.0",
        "status": "online",
        "message": "Forgeon Rust v1 playground",
        "endpoints": {
            "cats_list": "/v1/cats",
            "cats_detail": "/v1/cats/{id}",
            "status": "/status",
        }
    }))
}

pub async fn create_app() -> Router {
    logger::setup();

    let skip_db = std::env::var("SKIP_DB")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if skip_db {
        tracing::info!("SKIP_DB=true → skipping database initialization");
    } else if let Err(err) = models::sync_indexes().await {
        tracing::warn!("Database init failed, continuing anyway: {err}");
    }

    // v1 router: index + cats
    let v1_router = Router::new()
        .route("/", get(v1_index))
        .merge(routes::cat::create_route());

    Router::new()
        .merge(routes::status::create_route())
        .merge(routes::user::create_route())
        .merge(routes::pages::create_route()) // ⬅️ HTML pages we added
        .nest("/v1", v1_router)
        // High level logging of requests and responses
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        // CORS configuration. This should probably be more restrictive in
        // production.
        .layer(CorsLayer::permissive())

}
