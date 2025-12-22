use std::{net::SocketAddr, env};
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

#[cfg(test)]
mod tests;

use settings::SETTINGS;

/// Resolve the port in a "platform friendly" way:
/// 1) $PORT (Forgeon / Heroku / most PaaS)
/// 2) SETTINGS.server.port (your config)
fn resolve_port() -> u16 {
    if let Ok(p) = env::var("PORT") {
        if let Ok(n) = p.parse::<u16>() {
            return n;
        }
        // If PORT exists but invalid, we still continue with config
        // (but we log it so devs know what's wrong).
        info!("⚠️  Invalid PORT env var value: {:?}. Falling back to config.", p);
    }
    SETTINGS.server.port
}

/// Graceful shutdown handler for containers:
/// - SIGTERM is what orchestrators send on deploy/stop
/// - SIGINT for local Ctrl+C
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut term = signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        let mut int = signal(SignalKind::interrupt())
            .expect("failed to install SIGINT handler");

        tokio::select! {
            _ = term.recv() => {
                info!("🛑 SIGTERM received, starting graceful shutdown...");
            }
            _ = int.recv() => {
                info!("🛑 SIGINT received, starting graceful shutdown...");
            }
        }
    }

    #[cfg(not(unix))]
    {
        // Windows fallback: Ctrl+C
        let _ = tokio::signal::ctrl_c().await;
        info!("🛑 Ctrl+C received, starting graceful shutdown...");
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // If your logger module sets up tracing, keep it here:
    // logger::init();

    let port = resolve_port();

    // ✅ Critical for containers: bind to 0.0.0.0 so traffic can reach us.
    let address: SocketAddr = format!("{}:{}", SETTINGS.server.host, SETTINGS.server.port)
        .parse()
        .expect("Invalid server.host/server.port");

    let app = app::create_app().await;

    let listener = TcpListener::bind(address).await?;
    info!("🚀 Server listening on http://{}", &address);
    info!("ℹ️  Using PORT={}, config_port={}", port, SETTINGS.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}
