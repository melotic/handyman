use std::{fs, process::exit};

use clap::Parser;
use color_eyre::{eyre::Context, owo_colors::OwoColorize, Help, Result};
use commands::systemd;

pub mod cli;
pub mod commands;
pub mod config;
pub mod service;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = cli::Cli::parse();

    match cli.command() {
        cli::Command::CheckConfig {
            config,
            print_syntax,
        } => {
            let config_str = fs::read_to_string(config)
                .with_context(|| "failed to read the configuration file")
                .with_suggestion(|| "ensure that the configuration file exists")?;

            let status = commands::check_config::check_config(&config_str, print_syntax);

            if status.is_ok() {
                println!("{config} {ok} ✅", ok = "ok".bold().green());
            } else {
                println!("{config} {error} ❌", error = "error".bold().red());
                println!("{}", status.as_ref().unwrap_err());
            }

            exit(status.map(|_| 0).unwrap_or(1));
        }
        cli::Command::Systemd => {
            tracing_subscriber::fmt::init();
            systemd::run_service()?;
        }
    }

    Ok(())
}
