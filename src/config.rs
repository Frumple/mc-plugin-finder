use std::sync::OnceLock;

use config::Config;

const ENV_VAR_PREFIX: &str = "MCPF";

fn config() -> &'static config::Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(config::Environment::with_prefix(ENV_VAR_PREFIX).separator("_"))
            .build()
            .expect("config could not be built")
    })
}

pub fn get_config_string(key: &str) -> String {
    config().get_string(key).unwrap_or_else(|_| panic!("config key '{}' is not set to a string", key))
}