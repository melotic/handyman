use std::{process::Command, sync::Arc, time::Duration};

use crate::{
    config::{Configuration, Handler},
    service::{healthcheck::HealthCheck, healthchecks::http::HttpHealthCheck},
};

use crate::config::HealthCheckState;
use futures::future;
use tokio::{runtime::Builder, time::sleep};
use tracing::error;
use tracing::info;

use super::healthcheck::HealthCheckName;

macro_rules! run_healthchecks {
    ($config:ident, $config_name:ident, $healthcheck_name:ident) => {
        if let Some(healthchecks) = $config.$config_name() {
            info!(
                "[{}] Running {} healthchecks for {}",
                stringify!($config_name),
                healthchecks.len(),
                $config.name().unwrap_or("unnamed")
            );

            let healthcheck = $healthcheck_name::default();
            let results =
                get_healthcheck_results(healthcheck, healthchecks, stringify!($config_name)).await;

            let any_failed = results
                .iter()
                .any(|result| result == &HealthCheckState::Failed);
            let any_success = results.iter().any(|result| result == &HealthCheckState::Ok);

            for handler in $config.handlers() {
                if any_failed && handler.state() == HealthCheckState::Failed {
                    run_handler(handler).await;
                } else if any_success && handler.state() == HealthCheckState::Ok {
                    run_handler(handler).await;
                }
            }
        }
    };
}

pub fn start_service(configs: Vec<Configuration>) {
    info!("Creating async runtime");
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.block_on(async move {
        start_service_async(configs).await;
    });
}

async fn start_service_async(configs: Vec<Configuration>) {
    // TODO: use inotify + oneshot channel for hot reloads
    let configs = configs.into_iter().map(Arc::new).collect::<Vec<_>>();
    let mut futures = Vec::with_capacity(configs.len());

    for config in &configs {
        info!(
            "Launching health check: {name}",
            name = config.name().unwrap_or("unnamed")
        );
        futures.push(tokio::spawn(run_config(config.clone())));
    }

    future::join_all(futures).await;
}

async fn run_config(config: Arc<Configuration>) {
    loop {
        if let Some(duration) = config.interval() {
            sleep(Duration::from_secs(duration)).await;
        }

        run_healthchecks!(config, http, HttpHealthCheck);
    }
}

async fn run_handler(handler: &Handler) {
    info!(
        "Running {} handler command: {command}",
        handler.name().unwrap_or("unnamed"),
        command = handler.command()
    );

    let command = handler.command();

    // launch the command from a shell
    let output = match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => output,
        Err(error) => {
            error!("Failed to run command: {error}");
            return;
        }
    };

    if !output.status.success() {
        error!("Command failed: {command}");
    }
}

async fn get_healthcheck_results<H: HealthCheck>(
    healthcheck: H,
    configs: &[H::Config],
    name: &str,
) -> Vec<HealthCheckState>
where
    H::Config: HealthCheckName,
{
    let mut results = Vec::with_capacity(configs.len());

    for healthcheck_config in configs {
        // create log span
        let handler_name = healthcheck_config.name().unwrap_or("unnamed");
        let span = tracing::info_span!("running", handler_name, name);
        let _enter = span.enter();

        let check_status = healthcheck.check(healthcheck_config).await;
        info!("Returned {:?}", check_status);

        results.push(check_status);
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::config::Http;

    use super::*;

    #[tokio::test]
    async fn test_get_healthcheck_results() {
        let healthcheck = HttpHealthCheck::default();
        let configs = vec![Http::new(
            Some("test".to_string()),
            "invalid url".to_string(),
            Some(1),
        )];
        let results = get_healthcheck_results(healthcheck, &configs, "http").await;
        assert_eq!(results.len(), 1);
    }
}
