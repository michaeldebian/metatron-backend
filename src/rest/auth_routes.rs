use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{self, jwt::TeamClaim},
    db, rbac, AppState,
};

// ── Signup ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub org_name: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub name: String,
    pub is_owner: bool,
    pub teams: Vec<TeamResponse>,
    pub visible_modules: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamResponse {
    pub team_id: Uuid,
    pub name: String,
    pub role: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    if req.email.is_empty() || req.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email required and password must be at least 8 characters".to_string(),
        ));
    }

    if db::find_user_by_email(&state.db, &req.email)
        .await
        .map_err(internal)?
        .is_some()
    {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }

    let password_hash = auth::hash_password(&req.password).map_err(internal)?;

    let slug = req
        .org_name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();

    let (org_id, user_id) = db::create_org_with_owner(
        &state.db, &req.org_name, &slug, &req.email, &req.name, &password_hash,
    )
    .await
    .map_err(internal)?;

    let permissions =
        rbac::resolve_user_permissions(&state.db, user_id, org_id, true)
            .await
            .map_err(internal)?;

    let tokens = auth::create_token_pair(
        user_id, org_id, &req.email, &req.name, true,
        vec![], &permissions,
        &state.config.jwt_secret,
        state.config.jwt_expiry_secs,
        state.config.jwt_refresh_expiry_secs,
    )
    .map_err(internal)?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: tokens.expires_at,
        user: UserResponse {
            id: user_id, org_id,
            email: req.email, name: req.name,
            is_owner: true, teams: vec![],
            visible_modules: permissions.visible_modules(),
        },
    }))
}

// ── Login ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = db::find_user_by_email(&state.db, &req.email)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let password_hash = user
        .password_hash
        .as_deref()
        .ok_or((StatusCode::UNAUTHORIZED, "SSO-only account".to_string()))?;

    let valid = auth::verify_password(&req.password, password_hash).map_err(internal)?;
    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    db::touch_login(&state.db, user.id).await.ok();

    let permissions =
        rbac::resolve_user_permissions(&state.db, user.id, user.org_id, user.is_org_owner)
            .await
            .map_err(internal)?;

    // Get team memberships
    let team_rows: Vec<(Uuid, String, String)> = sqlx::query_as(
        "SELECT t.id, t.name, tm.role FROM teams t JOIN team_members tm ON tm.team_id = t.id WHERE tm.user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await
    .map_err(internal)?;

    let teams: Vec<TeamClaim> = team_rows
        .iter()
        .map(|(id, _, role)| TeamClaim { team_id: *id, role: role.clone() })
        .collect();

    let team_responses: Vec<TeamResponse> = team_rows
        .iter()
        .map(|(id, name, role)| TeamResponse { team_id: *id, name: name.clone(), role: role.clone() })
        .collect();

    let tokens = auth::create_token_pair(
        user.id, user.org_id, &user.email, &user.name, user.is_org_owner,
        teams, &permissions,
        &state.config.jwt_secret,
        state.config.jwt_expiry_secs,
        state.config.jwt_refresh_expiry_secs,
    )
    .map_err(internal)?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: tokens.expires_at,
        user: UserResponse {
            id: user.id, org_id: user.org_id,
            email: user.email, name: user.name,
            is_owner: user.is_org_owner,
            teams: team_responses,
            visible_modules: permissions.visible_modules(),
        },
    }))
}

// ── Refresh ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let claims = auth::verify_token(&req.refresh_token, &state.config.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))?;

    let permissions =
        rbac::resolve_user_permissions(&state.db, claims.sub, claims.org, claims.is_owner)
            .await
            .map_err(internal)?;

    let teams = claims.teams.clone();

    let tokens = auth::create_token_pair(
        claims.sub, claims.org, &claims.email, &claims.name, claims.is_owner,
        teams.clone(), &permissions,
        &state.config.jwt_secret,
        state.config.jwt_expiry_secs,
        state.config.jwt_refresh_expiry_secs,
    )
    .map_err(internal)?;

    let team_responses: Vec<TeamResponse> = teams
        .iter()
        .map(|t| TeamResponse { team_id: t.team_id, name: String::new(), role: t.role.clone() })
        .collect();

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: tokens.expires_at,
        user: UserResponse {
            id: claims.sub, org_id: claims.org,
            email: claims.email, name: claims.name,
            is_owner: claims.is_owner,
            teams: team_responses,
            visible_modules: permissions.visible_modules(),
        },
    }))
}

fn internal(e: impl std::fmt::Display) -> (StatusCode, String) {
    tracing::error!("Internal error: {e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
}
