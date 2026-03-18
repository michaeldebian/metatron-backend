use std::time::Duration;

use tonic::transport::Channel;
use tracing::{info, warn};

use super::proto::finops::fin_ops_service_client::FinOpsServiceClient;

/// gRPC client for the FinOpsService backend.
pub struct FinOpsClient {
    inner: Option<FinOpsServiceClient<Channel>>,
}

impl FinOpsClient {
    /// Connect to the FinOps service. Falls back gracefully if unavailable.
    pub async fn connect(url: &str, deadline: Duration) -> Self {
        match Channel::from_shared(url.to_string())
            .expect("valid URL")
            .timeout(deadline)
            .connect()
            .await
        {
            Ok(channel) => {
                info!(url, "Connected to FinOpsService");
                Self {
                    inner: Some(FinOpsServiceClient::new(channel)),
                }
            }
            Err(e) => {
                warn!(url, error = %e, "Failed to connect to FinOpsService — using stubs");
                Self { inner: None }
            }
        }
    }

    /// Get the inner client if connected.
    pub fn client(&self) -> Option<&FinOpsServiceClient<Channel>> {
        self.inner.as_ref()
    }

    /// Check if connected to a real backend.
    pub fn is_connected(&self) -> bool {
        self.inner.is_some()
    }
}
