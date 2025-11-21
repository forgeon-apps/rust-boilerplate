// src/routes/pages.rs
use axum::{
    response::Html,
    routing::get,
    Router,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/info", get(info_page))
        .route("/about", get(about_page))
        .route("/framework", get(framework_page))
}

// ───────────────── HANDLERS ─────────────────

async fn info_page() -> Html<&'static str> {
    Html(r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Forgeon · Rust Playground · Info</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      :root {
        color-scheme: dark light;
        --bg: #050505;
        --fg: #f5f5f5;
        --muted: #9ca3af;
        --accent: #ffffff;
        --border: #27272a;
        --mono: "SF Mono", Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
        --sans: system-ui, -apple-system, BlinkMacSystemFont, "SF Pro Text", sans-serif;
      }
      *, *::before, *::after { box-sizing: border-box; }
      body {
        margin: 0;
        min-height: 100vh;
        font-family: var(--sans);
        background: radial-gradient(circle at top, #18181b 0, #020617 55%);
        color: var(--fg);
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
      }
      .card {
        width: 100%;
        max-width: 720px;
        border-radius: 0.9rem;
        border: 1px solid var(--border);
        background: rgba(10, 10, 10, 0.95);
        box-shadow:
          0 18px 60px rgba(0,0,0,0.8),
          0 0 0 1px rgba(255,255,255,0.02);
        padding: 1.75rem 1.6rem;
      }
      h1 {
        font-size: 1.35rem;
        letter-spacing: 0.18em;
        text-transform: uppercase;
        margin: 0 0 0.8rem 0;
        font-weight: 600;
      }
      h1 span {
        display: inline-block;
        padding: 0.25rem 0.75rem;
        border-radius: 999px;
        border: 1px solid var(--border);
        background: rgba(15,15,15,0.9);
      }
      p {
        margin: 0.5rem 0;
        line-height: 1.6;
        color: var(--muted);
        font-size: 0.9rem;
      }
      .meta {
        display: flex;
        flex-wrap: wrap;
        gap: 0.45rem;
        margin: 0.9rem 0 1.4rem;
        font-size: 0.72rem;
        text-transform: uppercase;
        letter-spacing: 0.18em;
        color: var(--muted);
      }
      .pill {
        border-radius: 999px;
        border: 1px solid var(--border);
        padding: 0.25rem 0.8rem;
        background: rgba(12,12,12,0.85);
      }
      code {
        font-family: var(--mono);
        font-size: 0.78rem;
        background: #020617;
        border-radius: 0.45rem;
        border: 1px solid #111827;
        padding: 0.6rem 0.75rem;
        display: block;
        margin-top: 0.6rem;
        color: var(--fg);
        white-space: pre-wrap;
      }
      a {
        color: var(--accent);
        text-decoration: none;
        border-bottom: 1px solid rgba(148,163,184,0.4);
        padding-bottom: 1px;
      }
      a:hover {
        border-color: var(--fg);
      }
      .links {
        display: flex;
        flex-wrap: wrap;
        gap: 0.6rem;
        margin-top: 1.1rem;
        font-size: 0.78rem;
      }
      .link-pill {
        border-radius: 999px;
        border: 1px solid var(--border);
        padding: 0.35rem 0.9rem;
        background: #020617;
      }
      .link-pill span {
        opacity: 0.7;
      }
      .mono {
        font-family: var(--mono);
      }
    </style>
  </head>
  <body>
    <main class="card">
      <h1><span>Forgeon · Rust playground</span></h1>

      <p>
        This service is a small <strong>Axum</strong> API used to test networking,
        health checks, and container behavior before wiring it into real Forgeon runtimes.
      </p>

      <div class="meta">
        <div class="pill">Mode: Playground</div>
        <div class="pill">Stack: Rust · Axum · Tokio</div>
        <div class="pill">Status: Online</div>
      </div>

      <p>Quick endpoints you can hit:</p>
      <code>
GET /status        → JSON health payload
GET /v1/cats       → Sample resource (when DB is enabled)
GET /info          → This info page
GET /about         → Short description of this demo
GET /framework     → Details about the Rust/Axum stack
      </code>

      <div class="links">
        <span class="link-pill">
          <span>Dashboard hint · </span>
          <span class="mono">/status</span>
        </span>
        <span class="link-pill">
          <span>Forgeon · </span>
          <a href="https://forgeon.io" target="_blank" rel="noreferrer">forgeon.io</a>
        </span>
      </div>
    </main>
  </body>
</html>
"#)
}

async fn about_page() -> Html<&'static str> {
    Html(r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Forgeon · About this Rust demo</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      :root {
        color-scheme: dark light;
        --bg: #050505;
        --fg: #f5f5f5;
        --muted: #9ca3af;
        --border: #27272a;
        --accent: #ffffff;
        --sans: system-ui, -apple-system, BlinkMacSystemFont, "SF Pro Text", sans-serif;
      }
      * { box-sizing: border-box; }
      body {
        margin: 0;
        min-height: 100vh;
        background: #020617;
        color: var(--fg);
        font-family: var(--sans);
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
      }
      .card {
        width: 100%;
        max-width: 640px;
        border-radius: 1rem;
        border: 1px solid var(--border);
        padding: 1.75rem 1.6rem;
        background: radial-gradient(circle at top, #111827 0, #020617 55%);
      }
      h1 {
        font-size: 1.4rem;
        margin: 0 0 0.75rem 0;
        letter-spacing: 0.08em;
        text-transform: uppercase;
      }
      p {
        margin: 0.4rem 0;
        font-size: 0.9rem;
        line-height: 1.7;
        color: var(--muted);
      }
      a {
        color: var(--accent);
        text-decoration: none;
        border-bottom: 1px solid rgba(148,163,184,0.5);
      }
      a:hover {
        border-color: var(--fg);
      }
      .tagline {
        font-size: 0.8rem;
        text-transform: uppercase;
        letter-spacing: 0.18em;
        color: var(--muted);
        margin-bottom: 1rem;
      }
      ul {
        margin: 0.75rem 0 0;
        padding-left: 1.2rem;
        font-size: 0.88rem;
        color: var(--muted);
      }
    </style>
  </head>
  <body>
    <article class="card">
      <div class="tagline">About this playground</div>
      <h1>Rust API wired for Forgeon</h1>

      <p>
        This tiny service is a <strong>Rust + Axum</strong> playground used to test how
        Forgeon talks to containers: health checks, routes, timeouts, and log streaming.
      </p>

      <p>
        It&apos;s not a real product API, just a safe sandbox. You can deploy this to Forgeon,
        hit the endpoints, and verify that:
      </p>

      <ul>
        <li>the container boots correctly,</li>
        <li>HTTP routing is working,</li>
        <li>logs and health checks behave as expected.</li>
      </ul>

      <p style="margin-top: 0.9rem;">
        Want the real platform? Visit
        <a href="https://forgeon.io" target="_blank" rel="noreferrer">forgeon.io</a>.
      </p>
    </article>
  </body>
</html>
"#)
}

async fn framework_page() -> Html<&'static str> {
    Html(r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Forgeon · Framework stack</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>
      :root {
        color-scheme: dark light;
        --bg: #020617;
        --fg: #f9fafb;
        --muted: #9ca3af;
        --border: #1f2933;
        --sans: system-ui, -apple-system, BlinkMacSystemFont, "SF Pro Text", sans-serif;
        --mono: "SF Mono", Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
      }
      * { box-sizing: border-box; }
      body {
        margin: 0;
        min-height: 100vh;
        background: linear-gradient(to bottom, #020617, #020617 45%, #0b1120 100%);
        color: var(--fg);
        font-family: var(--sans);
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
      }
      .shell {
        width: 100%;
        max-width: 740px;
        border-radius: 1.1rem;
        border: 1px solid var(--border);
        background: rgba(3,7,18,0.96);
        box-shadow: 0 18px 50px rgba(0,0,0,0.8);
        overflow: hidden;
      }
      .shell-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.55rem 0.9rem;
        background: #020617;
        border-bottom: 1px solid #111827;
        font-size: 0.75rem;
        color: var(--muted);
      }
      .dots {
        display: flex;
        gap: 0.3rem;
      }
      .dot {
        width: 0.55rem;
        height: 0.55rem;
        border-radius: 999px;
        background: #111827;
      }
      .body {
        padding: 1.6rem 1.4rem 1.7rem;
      }
      h1 {
        font-size: 1.2rem;
        margin: 0 0 0.6rem 0;
      }
      .subtitle {
        font-size: 0.85rem;
        color: var(--muted);
        margin-bottom: 1rem;
      }
      .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit,minmax(180px,1fr));
        gap: 1rem;
        margin-top: 1rem;
      }
      .card {
        border-radius: 0.9rem;
        border: 1px solid #111827;
        background: radial-gradient(circle at top left, #111827 0, #020617 55%);
        padding: 0.9rem 0.85rem;
      }
      .label {
        font-size: 0.7rem;
        text-transform: uppercase;
        letter-spacing: 0.16em;
        color: var(--muted);
        margin-bottom: 0.45rem;
      }
      .value {
        font-family: var(--mono);
        font-size: 0.8rem;
      }
      ul {
        margin: 0.5rem 0 0;
        padding-left: 1.1rem;
        font-size: 0.8rem;
        color: var(--muted);
      }
      a {
        color: var(--fg);
        text-decoration: none;
        border-bottom: 1px solid rgba(148,163,184,0.4);
      }
      a:hover {
        border-color: var(--fg);
      }
    </style>
  </head>
  <body>
    <section class="shell">
      <header class="shell-header">
        <div class="dots">
          <span class="dot"></span>
          <span class="dot"></span>
          <span class="dot"></span>
        </div>
        <span class="truncate">/framework · Rust · Axum · Tokio</span>
        <span></span>
      </header>
      <div class="body">
        <h1>Framework stack</h1>
        <p class="subtitle">
          This playground is built as a small <strong>HTTP API</strong> to validate how
          Forgeon talks to Rust services before wiring real workloads.
        </p>

        <div class="grid">
          <div class="card">
            <div class="label">Runtime</div>
            <div class="value">Rust · Tokio · async/await</div>
            <ul>
              <li>Non-blocking IO</li>
              <li>Good fit for lightweight APIs</li>
            </ul>
          </div>

          <div class="card">
            <div class="label">HTTP layer</div>
            <div class="value">Axum + tower-http</div>
            <ul>
              <li>Typed request handlers</li>
              <li>Tracing, compression, CORS</li>
            </ul>
          </div>

          <div class="card">
            <div class="label">Use in Forgeon</div>
            <div class="value">Health checks & smoke tests</div>
            <ul>
              <li>/status for readiness</li>
              <li>/info &amp; /about for HTML checks</li>
            </ul>
          </div>
        </div>

        <p style="margin-top:1.4rem;font-size:0.8rem;color:var(--muted);">
          You can deploy any Rust/Axum service to Forgeon — this one is just a friendly
          placeholder while the real gods of compute boot up in the background.
        </p>
      </div>
    </section>
  </body>
</html>
"#)
}
