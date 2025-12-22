use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, fmt};

pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to setup settings"));

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    /// Bind host/interface. In containers you want "0.0.0.0".
    /// In local dev you can still use "127.0.0.1" if you want.
    #[serde(default = "default_host")]
    pub host: String,

    /// Bind port. Overridable via $PORT (Forgeon/Heroku standard).
    pub port: u16,
}

fn default_host() -> String {
    // Default to 0.0.0.0 so it works in Docker/PaaS out of the box.
    // You can override with SERVER__HOST=127.0.0.1 for local-only behavior.
    "0.0.0.0".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// Your run environment label used by configs.
    /// Example: "development", "staging", "production"
    pub environment: String,

    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub auth: Auth,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // RUN_MODE selects which config file to load: config/{RUN_MODE}.*
        // Defaults to "development" for local dev.
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            // Base defaults
            .add_source(File::with_name("config/default"))
            // Optional environment-specific file (config/development, config/production, etc.)
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            // Optional local override (never commit this)
            .add_source(File::with_name("config/local").required(false))
            // Env overrides:
            // SERVER__PORT=8080 -> server.port
            // SERVER__HOST=0.0.0.0 -> server.host
            .add_source(Environment::default().separator("__"));

        // Platform standard: PORT
        // Make it explicit and safe: parse to u16 so we don’t freeze a bad value.
        if let Ok(port_raw) = env::var("PORT") {
            let port: u16 = port_raw.parse().map_err(|_| {
                ConfigError::Message(format!(
                    "Invalid PORT env var: {port_raw:?} (expected 1..65535)"
                ))
            })?;
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display a useful address for *actual binding*, not always localhost.
        write!(f, "http://{}:{}", self.host, self.port)
    }
}
