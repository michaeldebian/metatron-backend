use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use super::jwt::Claims;

/// Extractor that validates the `Authorization: Bearer <token>` header
/// and provides the authenticated user's claims to handlers.
#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl AuthUser {
    /// Check if user has a specific permission.
    pub fn can(&self, module: &str, action: &str) -> bool {
        if self.0.is_owner {
            return true;
        }
        self.0
            .permissions
            .get(module)
            .map(|actions| actions.iter().any(|a| a == action))
            .unwrap_or(false)
    }

    /// Get list of team IDs this user belongs to.
    pub fn team_ids(&self) -> Vec<uuid::Uuid> {
        self.0.teams.iter().map(|t| t.team_id).collect()
    }
}

/// Wrapper for JWT secret stored in request extensions.
#[derive(Clone)]
pub struct JwtSecret(pub String);

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid authorization format"))?;

        // Get JWT secret from extensions (set by middleware layer)
        let secret = parts
            .extensions
            .get::<JwtSecret>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "JWT secret not configured"))?;

        let claims = super::jwt::verify_token(token, &secret.0)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(AuthUser(claims))
    }
}
