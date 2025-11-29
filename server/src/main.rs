use config::*;
use tracing::{instrument::WithSubscriber, *};


mod types;
use types::JTankDef;

fn main() {
    let tracing_sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)
        .expect("setting default subscriber failed");

    use std::path::PathBuf;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let config_path = root.join("src/config.toml");
    let tank_defs_path = root.join("src/tank_defs.json5");

    let config = config::Config::builder()
        .add_source(config::File::from(config_path))
        .build()
        .unwrap();

}
