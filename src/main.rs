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

            match &status {
                Ok(_) => {
                    println!("{config} {ok} ✅", ok = "ok".bold().green());
                }
                Err(e) => {
                    println!("{config} {error} ❌", error = "error".bold().red());
                    println!("{e}");
                }
            }

            exit(if status.is_ok() { 0 } else { 1 });
        }
        cli::Command::Systemd => {
            systemd::run_service()?;
        }
    }

    Ok(())
}
