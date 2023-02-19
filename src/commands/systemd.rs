use std::path::{Path, PathBuf};

use crate::config::Configuration;
use color_eyre::{eyre::Context, Help, Result};
use tracing::{error, info, metadata::LevelFilter, warn};
use tracing_subscriber::prelude::*;

const CONFIG_DIR: &str = "/etc/handyman/config.d";

pub fn run_service() -> Result<()> {
    // install jouranld tracing
    setup_tracing();

    info!("Starting Handyman service");

    let configurations = read_configs()?;

    Ok(())
}

fn setup_tracing() {
    let stdout = tracing_subscriber::fmt::layer().pretty();

    let journald = tracing_journald::layer().ok();

    let subscriber = tracing_subscriber::registry()
        .with(journald.with_filter(LevelFilter::INFO))
        .with(stdout);

    tracing::subscriber::set_global_default(subscriber).unwrap();
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
        // If we can't read get the entry, we'll just skip it and warn the user
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                error!("Failed to read configuration directory entry: {error}",);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            warn!(
                "Ignoring directory {path} in configuration directory",
                path = path.display()
            );
            continue;
        }

        let parsed_config = try_parse_config(&path);

        if let Ok(config) = parsed_config {
            configs.push(config);
            info!("Loaded configuration {path}", path = path.display());
        } else {
            error!(
                "Failed to load configuration {path}: {error}",
                path = path.display(),
                error = parsed_config.unwrap_err()
            );
        }
    }

    Ok(configs)
}

fn try_parse_config(path: &PathBuf) -> Result<Configuration> {
    let config = std::fs::read_to_string(path)
        .with_context(|| "failed to read the configuration file")
        .with_suggestion(|| "ensure that the configuration file exists")?;

    let config: Configuration =
        toml::from_str(&config).with_context(|| "failed to parse the configuration file")?;

    Ok(config)
}
