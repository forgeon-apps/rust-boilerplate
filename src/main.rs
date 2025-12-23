use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod app;
mod database;
mod errors;
mod logger;
mod models;
mod routes;
mod settings;
mod utils;
mod pages;

// There are a couple approaches to take when implementing E2E tests. This
// approach adds tests on /src/tests, this way tests can reference modules
// inside the src folder. Another approach would be to have the tests in a
// /tests folder on the root of the project, to do this and be able to import
// modules from the src folder, modules need to be exported as a lib.
#[cfg(test)]
mod tests;

use errors::Error;
use settings::SETTINGS;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 1. Try to get PORT from environment, fallback to your SETTINGS
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(SETTINGS.server.port);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let app = app::create_app().await;

    let listener = TcpListener::bind(address).await?;
    
    // Use println! here to ensure you see this in logs 
    // even if tracing is not fully initialized yet
    println!("🚀 Server started on {}", address);

    axum::serve(listener, app).await
}