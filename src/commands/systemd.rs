use std::path::Path;

use crate::config::Configuration;
use color_eyre::{eyre::Context, Help, Result};
use tracing::{info, warn};

const CONFIG_DIR: &str = "/etc/handyman/config.d";

pub fn run_service() -> Result<()> {
    info!("Starting Handyman service");

    let configurations = read_configs()?;

    Ok(())
}

fn read_configs() -> Result<Vec<Configuration>> {
    info!("Reading configurations from {CONFIG_DIR}");

    let mut configs = vec![];
    let config_path = Path::new(CONFIG_DIR);

    if !config_path.exists() {
        warn!("Config directory not found, creating {CONFIG_DIR}");
        std::fs::create_dir_all(CONFIG_DIR)
            .with_context(|| "failed to create the configuration directory")
            .with_suggestion(|| {
                "ensure handyman has suitable permissions to create the directory"
            })?;

        return Ok(configs);
    }

    for entry in std::fs::read_dir(config_path)
        .with_context(|| "failed to read the configuration directory")
        .with_suggestion(|| "ensure that the configuration directory exists")?
    {
        let entry = entry
            .with_context(|| "failed to read the configuration directory")
            .with_suggestion(|| "ensure that the configuration directory exists")?;

        let path = entry.path();

        if path.is_dir() {
            warn!(
                "Ignoring directory {path} in configuration directory",
                path = path.display()
            );
            continue;
        }

        let config = std::fs::read_to_string(&path)
            .with_context(|| "failed to read the configuration file")
            .with_suggestion(|| "ensure that the configuration file exists")?;

        let config: Configuration =
            toml::from_str(&config).with_context(|| "failed to parse the configuration file")?;

        configs.push(config);
    }

    Ok(configs)
}
