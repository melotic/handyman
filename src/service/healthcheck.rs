use async_trait::async_trait;

use crate::config::HealthCheckState;

#[async_trait]
pub trait HealthCheck {
    type Config;

    async fn check(&self, config: &Self::Config) -> HealthCheckState;
}
