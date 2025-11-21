# rustapi

[![Tests](https://github.com/ndelvalle/rustapi/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/ndelvalle/rustapi/actions/workflows/test.yml)

RESTful API template built with Rust lang. It uses [MongoDB](https://docs.mongodb.com/)
database and [Axum](https://github.com/tokio-rs/axum) HTTP framework.

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://docs.mongodb.com/manual/installation/)

### How to use this template

To use this template as your project starting point, click "Use this template" at the top of this page, or click [here](https://github.com/ndelvalle/rustapi/generate).

### Feature highlights

* Authentication. Based on [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
* Layered configuration. Based on [config-rs](https://github.com/mehcode/config-rs)
* Logs. Based on [tracing](https://github.com/tokio-rs/tracing)
* Error handling
* Pagination
* E2E Tests
* OpenAPI Specification
* CI based on Github actions
* Dependabot configuration

### Project structure

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
│   ├── default.json    # Default configuration
│   ├── production.json # Production configuration (Overwrites the default)
│   └── test.json       # Test configuration (Overwrites the default)
├── rustfmt.toml
├── src
│   ├── database.rs
│   ├── errors.rs
│   ├── lib             # Helpers not related to the business model
│   │   ├── authenticate_request.rs
│   │   ├── date.rs
│   │   ├── mod.rs
│   │   ├── models.rs   # Base Database Model trait
│   │   ├── to_object_id.rs
│   │   └── token.rs
│   ├── logger.rs
│   ├── main.rs
│   ├── models
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── routes
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   ├── status.rs
│   │   └── user.rs
│   ├── settings.rs
│   └── tests           # E2E Tests
└── test.sh
```

````md

## Quick start (Playground mode – no DB)

This is the simplest way to run it locally or inside Forgeon as a demo service.

```bash
# 1) Clone the repo
git clone https://github.com/forgeon-apps/rust-boilerplate
cd rustapi

# 2) Run in playground mode (skips MongoDB initialization)
SKIP_DB=true cargo run
````

The server will start on:

```text
http://127.0.0.1:8080
```

### Playground routes

These are the routes you can hit in a browser or with curl:

#### HTML pages

* `GET /info`
  Service info page – what this app is, endpoints summary, and Forgeon note.

* `GET /about`
  Short “about this service” page, ideal for demo / landing.

* `GET /framework`
  Shows the tech stack: Axum, Tokio, tower-http, etc.

#### JSON / API

* `GET /status`
  Health / readiness endpoint (good for liveness/readiness probes).

* `GET /v1`
  JSON index of the v1 API, with links to key endpoints.

* `GET /v1/cats`
  Sample resource list (in playground mode the DB init is skipped, but route wiring stays the same).

> 💡 For Forgeon: this app is perfect as a **playground deployment** to test routing, health checks, and connectivity.

---

## Full API mode (with MongoDB)

To use the original MongoDB-backed features (auth, pagination, E2E tests):

1. Start MongoDB locally (default connection: `mongodb://localhost:27017` or as configured in `config/default.json`).
2. Make sure **`SKIP_DB` is unset** or set to `false`.
3. Run:

```bash
cargo run
```

The app will attempt to sync indexes on startup:

```rust
models::sync_indexes()
    .await
    .expect("Failed to sync database indexes");
```

### Tests

Tests depend on MongoDB being available.

```bash
make test
```

This will run:

```bash
cargo test -- --test-threads=1 --nocapture --color=always
```

---

## Using this template for your own project

To use the original template as a starting point on GitHub:

* Click **“Use this template”** at the top of the repo, or
* Go directly: `https://github.com/forgeon-apps/rust-boilerplate`

Then you can:

* Keep **playground mode** (`SKIP_DB=true`) for quick demos / Forgeon deployments.
* Enable MongoDB and the test suite when you’re ready for real data.

---

## Contributing

Contributions are welcome — feel free to:

* Fork and open a pull request
* File issues for bugs or improvements
* Suggest tweaks to the playground routes / HTML pages

Happy hacking 🦀🚀
