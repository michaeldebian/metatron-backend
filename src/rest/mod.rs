pub mod auth_routes;

use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};

use crate::{auth::middleware::JwtSecret, graphql::MetatronSchema, AppState};

/// Build the full HTTP router with REST + GraphQL endpoints.
pub fn router(state: AppState, schema: MetatronSchema) -> Router {
    let jwt_secret = JwtSecret(state.config.jwt_secret.clone());

    Router::new()
        // Health check
        .route("/health", get(health))
        // REST auth (no auth required)
        .route("/api/v1/auth/signup", post(auth_routes::signup))
        .route("/api/v1/auth/login", post(auth_routes::login))
        .route("/api/v1/auth/refresh", post(auth_routes::refresh))
        // GraphQL
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        // State & extensions
        .layer(Extension(jwt_secret))
        .with_state(state)
        .layer(Extension(schema))
}

async fn health() -> &'static str {
    "ok"
}

async fn graphql_playground() -> axum::response::Html<&'static str> {
    axum::response::Html(
        r#"<!DOCTYPE html>
<html>
<head><title>Metatron GraphQL</title>
<link rel="stylesheet" href="https://unpkg.com/graphiql/graphiql.min.css" />
</head>
<body style="margin:0">
<div id="graphiql" style="height:100vh"></div>
<script crossorigin src="https://unpkg.com/react/umd/react.production.min.js"></script>
<script crossorigin src="https://unpkg.com/react-dom/umd/react-dom.production.min.js"></script>
<script crossorigin src="https://unpkg.com/graphiql/graphiql.min.js"></script>
<script>
  const root = ReactDOM.createRoot(document.getElementById('graphiql'));
  root.render(React.createElement(GraphiQL, {
    fetcher: GraphiQL.createFetcher({ url: '/graphql' }),
    defaultEditorToolsVisibility: true,
  }));
</script>
</body></html>"#,
    )
}

async fn graphql_handler(
    Extension(schema): Extension<MetatronSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
