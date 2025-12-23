use axum::http::header;
use axum::{Router};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

use crate::logger;
use crate::models;
use crate::routes;

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

pub async fn create_app() -> Router {
    logger::setup();

    // ✅ default: no DB. Enable DB with USE_DB=1
    let use_db = env_truthy("USE_DB");
    let skip_db = !use_db;

    let mut app = Router::new()
        .merge(routes::public::create_route())
        .merge(routes::status::create_route())
        .merge(Router::new().nest("/v1", Router::new()));

    if skip_db {
        tracing::warn!("🟡 DB disabled (set USE_DB=1 to enable). Skipping Mongo init + DB routes");
    } else {
        if let Err(e) = models::sync_indexes().await {
            tracing::error!(error=%e, "🔴 DB init failed; continuing without DB routes (set USE_DB=0 to skip)");
        } else {
            app = app
                .merge(routes::user::create_route())
                .merge(Router::new().nest(
                    "/v1",
                    Router::new().merge(routes::cat::create_route()),
                ));
        }
    }

    app.layer(
        trace::TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
            .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
    )
    .layer(SetSensitiveHeadersLayer::new(std::iter::once(header::AUTHORIZATION)))
    .layer(CompressionLayer::new())
    .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")))
    .layer(CorsLayer::permissive())
}
