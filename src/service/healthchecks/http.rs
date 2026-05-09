use std::time::Duration;

use crate::{
    config::{HealthCheckState, Http},
    service::healthcheck::{HealthCheck, HealthCheckName},
};
use async_trait::async_trait;
use tracing::{debug, error, info_span, Instrument};

pub struct HttpHealthCheck {
    client: reqwest::Client,
}

impl Default for HttpHealthCheck {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl HttpHealthCheck {
    async fn check_inner(&self, config: &Http) -> HealthCheckState {
        let request = self
            .client
            .get(config.url())
            .timeout(Duration::from_secs(config.timeout().unwrap_or(u64::MAX)));

        debug!(request = debug(&request));

        let response = request.send().await;

        debug!(response = debug(&response));

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    HealthCheckState::Ok
                } else {
                    HealthCheckState::Failed
                }
            }
            Err(error) => {
                error!("Failed to make request: {error}");
                HealthCheckState::Failed
            }
        }
    }
}

#[async_trait]
impl HealthCheck for HttpHealthCheck {
    type Config = Http;

    async fn check(&self, config: &Self::Config) -> HealthCheckState {
        self.check_inner(config)
            .instrument(info_span!(
                "http_healthcheck",
                url = config.url(),
                name = config.name()
            ))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HealthCheckState;
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_http_healthcheck() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let healthcheck = HttpHealthCheck::default();
        let config = Http::new(Some("test".to_string()), format!("{}/", server.uri()), None);

        let result = healthcheck.check_inner(&config).await;
        assert_eq!(result, HealthCheckState::Ok);
    }

    #[tokio::test]
    async fn test_http_healthcheck_failed() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let healthcheck = HttpHealthCheck::default();
        let config = Http::new(Some("test".to_string()), format!("{}/", server.uri()), None);

        let result = healthcheck.check_inner(&config).await;
        assert_eq!(result, HealthCheckState::Failed);
    }

    #[tokio::test]
    async fn test_http_healthcheck_timeout() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
            .mount(&server)
            .await;

        let healthcheck = HttpHealthCheck::default();
        let config = Http::new(
            Some("test".to_string()),
            format!("{}/", server.uri()),
            Some(1),
        );

        let result = healthcheck.check_inner(&config).await;
        assert_eq!(result, HealthCheckState::Failed);
    }

    #[tokio::test]
    async fn test_http_healthcheck_timeout_disabled() {
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(100)))
            .mount(&server)
            .await;

        let healthcheck = HttpHealthCheck::default();
        let config = Http::new(Some("test".to_string()), format!("{}/", server.uri()), None);

        let result = healthcheck.check_inner(&config).await;
        assert_eq!(result, HealthCheckState::Ok);
    }

    #[tokio::test]
    async fn test_http_healthcheck_invalid_url() {
        let healthcheck = HttpHealthCheck::default();

        let config = Http::new(Some("test".to_string()), "not a url".to_string(), None);

        let result = healthcheck.check_inner(&config).await;

        assert_eq!(result, HealthCheckState::Failed);
    }
}
