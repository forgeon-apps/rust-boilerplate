use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{env, fmt};

pub static SETTINGS: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to setup settings"));

fn default_environment() -> String {
    "development".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_logger_level() -> String {
    "info".to_string()
}

fn default_db_uri() -> String {
    // harmless default; you can override via env when USE_DB=1
    "mongodb://localhost:27017".to_string()
}

fn default_db_name() -> String {
    "app".to_string()
}

fn default_auth_secret() -> String {
    // fine for demo/playground; override in production
    "dev-secret-change-me".to_string()
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Server {
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Logger {
    #[serde(default = "default_logger_level")]
    pub level: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Database {
    #[serde(default = "default_db_uri")]
    pub uri: String,

    #[serde(default = "default_db_name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Auth {
    #[serde(default = "default_auth_secret")]
    pub secret: String,
}

// Remove the #[allow(dead_code)] attribute from the Settings struct when all the fields are being used.
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default = "default_environment")]
    pub environment: String,

    #[serde(default)]
    pub server: Server,

    #[serde(default)]
    pub logger: Logger,

    #[serde(default)]
    pub database: Database,

    #[serde(default)]
    pub auth: Auth,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            environment: default_environment(),
            server: Server::default(),
            logger: Logger::default(),
            database: Database::default(),
            auth: Auth::default(),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        // ✅ start from defaults so deserialization never fails when files are missing
        let mut builder = Config::builder()
            .set_default("environment", default_environment())?
            .set_default("server.port", default_port())?
            .set_default("logger.level", default_logger_level())?
            .set_default("database.uri", default_db_uri())?
            .set_default("database.name", default_db_name())?
            .set_default("auth.secret", default_auth_secret())?
            // ✅ make default config optional for container/runtime environments
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"));

        // Some cloud services like Heroku/Forgeon expose a randomly assigned port in PORT
        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "http://localhost:{}", &self.port)
    }
}
