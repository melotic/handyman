use std::{fs::File, os::unix::prelude::PermissionsExt};

use crate::commands::check_config::eyre::eyre;
use crate::config::Configuration;
use color_eyre::{
    eyre::{self, Context},
    Help, Report, Result,
};

pub fn check_config(config: &str, print_syntax: &bool) -> Result<()> {
    let config: Configuration = toml::from_str(config)
        .map_err(|e| Report::msg(e.to_string().trim_end().to_string()))
        .with_context(|| "failed to parse the configuration file")?;

    if let Some(handlers) = config.handlers() {
        for handler in handlers {
            let command = handler.command();

            // Check if the command exists, taking care that there could be arguments passed into it
            let command = command
                .split_whitespace()
                .next()
                .ok_or_else(|| eyre!("{} is not a valid command", command))?;

            // get the full path
            let command = which::which(command)
                .with_context(|| "failed to get the full path of the command")
                .with_suggestion(|| "ensure that the command exists")?;

            // check if that file exists
            if !command.exists() {
                return Err(eyre!("the command {} does not exist", command.display()))
                    .with_suggestion(|| "ensure that the command exists");
            }

            // check if the file is executable
            let metadata = File::open(&command)
                .with_context(|| "failed to open the command file")
                .with_suggestion(|| "ensure that the command file exists")?
                .metadata()
                .with_context(|| "failed to get the metadata of the command file")
                .with_suggestion(|| "ensure that the command file exists")?;

            if !metadata.permissions().mode() & 0o111 == 0o111 {
                return Err(eyre!("the command {} is not executable", command.display()))
                    .with_suggestion(|| "ensure that the command is executable");
            }
        }
    }

    if *print_syntax {
        println!("{config:#?}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_config() {
        let config = r#"
            name = "test"

            [[handlers]]
            name = "test"
            command = "echo"
            state = "ok"
            timeout = 10
        "#;

        assert!(check_config(config, &false).is_ok());
    }

    #[test]
    fn test_check_config_invalid_command() {
        let config = r#"
            name = "test"

            [[handlers]]
            name = "test"
            command = "fake-test-command"
            state = "ok"
            timeout = 10
        "#;

        assert!(check_config(config, &false).is_err());
    }

    #[test]
    fn test_check_config_invalid_command_file() {
        let config = r#"
            name = "test"

            [[handlers]]
            name = "test"
            command = "echo_test"
            state = "ok"
            timeout = 10
        "#;

        assert!(check_config(config, &false).is_err());
    }

    #[test]
    fn test_check_config_invalid_command_file_permissions() {
        let config = r#"
            name = "test"

            [[handlers]]
            name = "test"
            command = "/etc/passwd"
            state = "ok"
            timeout = 10
        "#;

        assert!(check_config(config, &false).is_err());
    }
}
