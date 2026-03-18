use std::time::Duration;

use tonic::transport::Channel;
use tracing::{info, warn};

use super::proto::cluster::cluster_service_client::ClusterServiceClient;

/// gRPC client for the ClusterService backend.
pub struct ClusterClient {
    inner: Option<ClusterServiceClient<Channel>>,
}

impl ClusterClient {
    /// Connect to the cluster service. Falls back gracefully if unavailable.
    pub async fn connect(url: &str, deadline: Duration) -> Self {
        match Channel::from_shared(url.to_string())
            .expect("valid URL")
            .timeout(deadline)
            .connect()
            .await
        {
            Ok(channel) => {
                info!(url, "Connected to ClusterService");
                Self {
                    inner: Some(ClusterServiceClient::new(channel)),
                }
            }
            Err(e) => {
                warn!(url, error = %e, "Failed to connect to ClusterService — using stubs");
                Self { inner: None }
            }
        }
    }

    /// Get the inner client if connected.
    pub fn client(&self) -> Option<&ClusterServiceClient<Channel>> {
        self.inner.as_ref()
    }

    /// Check if connected to a real backend.
    pub fn is_connected(&self) -> bool {
        self.inner.is_some()
    }
}
