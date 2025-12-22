use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::pages;

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(home_page))
        .route("/info", get(info_page))
        .route("/about", get(about_page))
        .route("/framework", get(framework_page))
        .route("/v1/status", get(status_view))
        .route("/v1", get(v1_index))
        .route("/api/data", get(sample_data))
        .route("/api/items/{item_id}", get(get_item))
}

async fn home_page() -> Html<String> {
    let body = r#"
        <div class="grid">
          <div>
            <div class="eyebrow">Forgeon · Axum playground</div>
            <h1>Rust Axum boilerplate, ready for deploy tests.</h1>
            <p>
              This service is a small Axum app you can use to test routing,
              health checks, JSON APIs, and HTML responses inside Forgeon or on your
              local machine.
            </p>

            <div class="pill-row">
              <div class="pill"><strong>GET</strong> /</div>
              <div class="pill">Axum · Tokio</div>
              <div class="pill">Playground mode (no DB required)</div>
            </div>
          </div>

          <div>
            <div class="links">
              <a href="/info">/info <span>– service overview & Forgeon context</span></a>
              <a href="/about">/about <span>– what this boilerplate is for</span></a>
              <a href="/framework">/framework <span>– stack: Axum, Rust</span></a>
              <a href="/status">/status <span>– JSON health endpoint</span></a>
              <a href="/v1">/v1 <span>– JSON API index</span></a>
            </div>
          </div>
        </div>

        <div class="meta">
          <span>Deploy this to Forgeon as a simple Axum playground.</span>
          <span class="badge">127.0.0.1</span>
        </div>
    "#;

    Html(pages::html_shell("Forgeon Axum playground", body))
}

async fn info_page() -> Html<String> {
    let body = r#"
        <div class="back"><a href="/">← Back to home</a></div>

        <div class="grid">
          <div>
            <div class="eyebrow">Service info</div>
            <h1>axum-playground · Forgeon-ready microservice</h1>
            <p>
              This instance exposes a couple of HTML & JSON endpoints so you can
              quickly verify that traffic is reaching the container correctly.
            </p>

            <ul>
              <li>Check that the container boots and responds.</li>
              <li>Wire health checks to <code>/status</code>.</li>
              <li>Inspect logs and latency from Forgeon.</li>
            </ul>
          </div>

          <div>
            <div class="links">
              <a href="/">/ <span>– landing page</span></a>
              <a href="/v1">/v1 <span>– JSON index</span></a>
            </div>
          </div>
        </div>
    "#;

    Html(pages::html_shell("Forgeon · Axum Playground · Info", body))
}

async fn about_page() -> Html<String> {
    let body = r#"
        <div class="back"><a href="/">← Back to home</a></div>

        <div>
          <div class="eyebrow">About this playground</div>
          <h1>Axum app wired for Forgeon.</h1>
          <p>
            This tiny service is an <strong>Axum</strong> playground used to test how
            Forgeon talks to containers: health checks, routes, timeouts, and logs.
          </p>
          <p>
            It&apos;s not a production API – just a safe sandbox you can deploy, poke, and
            then replace with your real service once everything feels right.
          </p>

          <ul>
            <li>Boots fast with no database configured.</li>
            <li>Has HTML endpoints for visual checks.</li>
            <li>Has JSON endpoints for programmatic checks.</li>
          </ul>
        </div>
    "#;

    Html(pages::html_shell("Forgeon · About this Axum demo", body))
}

async fn framework_page() -> Html<String> {
    let body = r#"
        <div class="back"><a href="/">← Back to home</a></div>

        <div class="grid">
          <div>
            <div class="eyebrow">Stack</div>
            <h1>Built with Axum & Rust.</h1>
            <p>
              The service uses <strong>Axum</strong> for routing, typically served by
              <strong>Tower</strong> middleware behind Forgeon. In playground mode it runs
              without any database.
            </p>
          </div>

          <div>
            <div class="links">
              <a href="/status">/status <span>– health JSON</span></a>
              <a href="/info">/info <span>– service overview</span></a>
              <a href="/v1">/v1 <span>– API index</span></a>
            </div>
          </div>
        </div>
    "#;

    Html(pages::html_shell("Forgeon · Axum Framework stack", body))
}

async fn status_view() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "axum-playground",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn v1_index() -> impl IntoResponse {
    Json(json!({
        "message": "Axum API v1 index",
        "service": "axum-playground",
        "routes": {
            "html": ["/", "/info", "/about", "/framework"],
            "health": ["/status"],
            "api": ["/v1", "/api/data", "/api/items/{item_id}"],
        }
    }))
}

async fn sample_data() -> impl IntoResponse {
    Json(json!({
        "data": [
            {"id": 1, "name": "Sample Item 1", "value": 100},
            {"id": 2, "name": "Sample Item 2", "value": 200},
            {"id": 3, "name": "Sample Item 3", "value": 300}
        ],
        "total": 3,
        "timestamp": "2024-01-01T00:00:00Z"
    }))
}

async fn get_item(Path(item_id): Path<i64>) -> impl IntoResponse {
    Json(json!({
        "item": {
            "id": item_id,
            "name": format!("Sample Item {}", item_id),
            "value": item_id * 100
        },
        "timestamp": "2024-01-01T00:00:00Z"
    }))
}
