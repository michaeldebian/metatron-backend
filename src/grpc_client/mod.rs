pub mod cluster;
pub mod finops;

pub use cluster::ClusterClient;
pub use finops::FinOpsClient;

/// Generated protobuf modules from tonic-build.
pub mod proto {
    pub mod cluster {
        tonic::include_proto!("metratron.cluster.v1");
    }
    pub mod finops {
        tonic::include_proto!("metratron.finops.v1");
    }
    pub mod mcp {
        tonic::include_proto!("metratron.mcp.v1");
    }
    pub mod auth {
        tonic::include_proto!("metratron.auth.v1");
    }
    pub mod pipeline {
        tonic::include_proto!("metratron.pipeline.v1");
    }
    pub mod analytics {
        tonic::include_proto!("metratron.analytics.v1");
    }
    pub mod report {
        tonic::include_proto!("metratron.report.v1");
    }
}
