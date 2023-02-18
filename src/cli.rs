use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Command {
    CheckConfig {
        config: String,

        #[clap(short, long, default_value_t = false)]
        print_syntax: bool,
    },
    Systemd,
}
