mod auth;
mod config;
mod databend;
mod db;
mod graphql;
mod grpc_client;
mod rbac;
mod rest;
mod websocket;

use std::future::IntoFuture;
use std::sync::Arc;

use anyhow::Result;
use config::Config;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Shared application state accessible from all handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: sqlx::PgPool,
    pub databend: Arc<databend::DatabendClient>,
    pub cluster_client: Arc<grpc_client::ClusterClient>,
    pub finops_client: Arc<grpc_client::FinOpsClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with(fmt::layer().with_writer(std::io::stderr))
        .init();

    let cfg = Config::from_env();
    info!(
        http_port = cfg.http_port,
        grpc_port = cfg.grpc_port,
        database_url = %mask_password(&cfg.database_url),
        cluster_url = %cfg.cluster_service_url,
        finops_url = %cfg.finops_service_url,
        "Starting Metatron API Server"
    );

    // ── Database connections ─────────────────────────────────────────────────
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.database_url)
        .await?;
    info!("PostgreSQL connected");

    // Run migrations from embedded SQL
    sqlx::raw_sql(include_str!("db/migrations/001_initial.sql"))
        .execute(&db)
        .await?;
    info!("Migrations applied");

    // Seed system permission sets
    rbac::seed_system_permission_sets(&db).await?;
    info!("System permission sets seeded");

    let databend = Arc::new(databend::DatabendClient::new(&cfg.databend_dsn));

    // ── gRPC backend clients ────────────────────────────────────────────────
    let cluster_client = Arc::new(
        grpc_client::ClusterClient::connect(&cfg.cluster_service_url, cfg.grpc_deadline).await,
    );
    let finops_client = Arc::new(
        grpc_client::FinOpsClient::connect(&cfg.finops_service_url, cfg.grpc_deadline).await,
    );

    let state = AppState {
        config: Arc::new(cfg.clone()),
        db,
        databend,
        cluster_client,
        finops_client,
    };

    // ── GraphQL schema ──────────────────────────────────────────────────────
    let schema = graphql::build_schema(state.clone());

    // ── HTTP server (REST + GraphQL) ────────────────────────────────────────
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = rest::router(state.clone(), schema).layer(cors);

    let http_addr: std::net::SocketAddr = format!("0.0.0.0:{}", state.config.http_port).parse()?;
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    let http_server = axum::serve(listener, app);

    info!("HTTP server on http://0.0.0.0:{}", state.config.http_port);
    info!("  GraphQL playground: http://0.0.0.0:{}/graphql", state.config.http_port);
    info!("  REST API:           http://0.0.0.0:{}/api/v1/", state.config.http_port);
    info!("  Health:             http://0.0.0.0:{}/health", state.config.http_port);

    http_server.into_future().await?;

    Ok(())
}

/// Mask password in connection strings for logging.
fn mask_password(url: &str) -> String {
    if let Some(at) = url.find('@') {
        if let Some(colon) = url[..at].rfind(':') {
            return format!("{}:***{}", &url[..colon], &url[at..]);
        }
    }
    url.to_string()
}
