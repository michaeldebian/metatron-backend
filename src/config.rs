use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    // Server ports
    pub http_port: u16,
    pub grpc_port: u16,

    // PostgreSQL
    pub database_url: String,

    // Databend
    pub databend_dsn: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_expiry_secs: u64,
    pub jwt_refresh_expiry_secs: u64,

    // Encryption key for cloud credentials at rest (hex-encoded 32 bytes)
    pub credential_encryption_key: String,

    // Upstream gRPC services
    pub cluster_service_url: String,
    pub finops_service_url: String,
    pub mcp_service_url: String,
    pub grpc_deadline: Duration,

    // External integrations
    pub teams_webhook_url: Option<String>,
    pub slack_webhook_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            http_port: port("HTTP_PORT", 8080),
            grpc_port: port("GRPC_PORT", 50060),

            database_url: env(
                "DATABASE_URL",
                "postgres://metatron:metatron@localhost:5432/metatron",
            ),

            databend_dsn: env(
                "DATABEND_DSN",
                "databend://root:@localhost:8000/default?sslmode=disable",
            ),

            jwt_secret: env("JWT_SECRET", "metatron-dev-secret-change-in-production"),
            jwt_expiry_secs: std::env::var("JWT_EXPIRY_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(900), // 15 minutes
            jwt_refresh_expiry_secs: std::env::var("JWT_REFRESH_EXPIRY_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(604800), // 7 days

            credential_encryption_key: env(
                "CREDENTIAL_ENCRYPTION_KEY",
                "0000000000000000000000000000000000000000000000000000000000000000",
            ),

            cluster_service_url: env("CLUSTER_SERVICE_URL", "http://localhost:50051"),
            finops_service_url: env("FINOPS_SERVICE_URL", "http://localhost:50053"),
            mcp_service_url: env("MCP_SERVICE_URL", "http://localhost:50050"),
            grpc_deadline: Duration::from_millis(
                std::env::var("GRPC_DEADLINE_MS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(5000),
            ),

            teams_webhook_url: std::env::var("TEAMS_WEBHOOK_URL").ok(),
            slack_webhook_url: std::env::var("SLACK_WEBHOOK_URL").ok(),
        }
    }
}

fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn port(key: &str, default: u16) -> u16 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
