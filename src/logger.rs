use std::env;

use crate::settings::SETTINGS;

pub fn setup() {
    if env::var_os("RUST_LOG").is_none() {
        // compile-time crate name (always available)
        let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("app");

        let level = SETTINGS.logger.level.as_str();
        let filter = format!("{app_name}={level},tower_http={level}");

        env::set_var("RUST_LOG", filter);
    }

    tracing_subscriber::fmt::init();
}
