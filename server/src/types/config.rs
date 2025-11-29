use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub tick_rate: f32,
}

impl Config {
    pub fn load() -> Result<Config, config::ConfigError> {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/config.json5");

        let text = std::fs::read_to_string(path).expect("failed to read config file!");
        let cfg: Config = serde_json5::from_str(&text).expect("failed to parse config!");

        CONFIG.set(cfg).expect("failed to set global config!");

        Ok(CONFIG.get().unwrap().clone())
    }

    pub fn get() -> &'static Config {
        CONFIG
            .get()
            .expect("could not get config. maybe uninitialized?")
    }
}
