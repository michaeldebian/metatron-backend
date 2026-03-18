use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rbac::types::ResolvedPermissions;

/// JWT claims stored in every access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — user ID
    pub sub: Uuid,
    /// Organization ID
    pub org: Uuid,
    pub email: String,
    pub name: String,
    pub is_owner: bool,
    /// Team memberships: [(team_id, role)]
    pub teams: Vec<TeamClaim>,
    /// Flattened permissions: module → [actions]
    pub permissions: std::collections::HashMap<String, Vec<String>>,
    /// Issued at (epoch seconds)
    pub iat: i64,
    /// Expiry (epoch seconds)
    pub exp: i64,
    /// Token ID for revocation
    pub jti: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamClaim {
    pub team_id: Uuid,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

/// Create an access + refresh token pair.
pub fn create_token_pair(
    user_id: Uuid,
    org_id: Uuid,
    email: &str,
    name: &str,
    is_owner: bool,
    teams: Vec<TeamClaim>,
    permissions: &ResolvedPermissions,
    secret: &str,
    access_expiry_secs: u64,
    refresh_expiry_secs: u64,
) -> anyhow::Result<TokenPair> {
    let now = Utc::now().timestamp();
    let access_exp = now + access_expiry_secs as i64;
    let refresh_exp = now + refresh_expiry_secs as i64;

    let perm_map = permissions.to_claims_map();

    let access_claims = Claims {
        sub: user_id,
        org: org_id,
        email: email.to_string(),
        name: name.to_string(),
        is_owner,
        teams: teams.clone(),
        permissions: perm_map.clone(),
        iat: now,
        exp: access_exp,
        jti: Uuid::new_v4(),
    };

    let refresh_claims = Claims {
        sub: user_id,
        org: org_id,
        email: email.to_string(),
        name: name.to_string(),
        is_owner,
        teams,
        permissions: perm_map,
        iat: now,
        exp: refresh_exp,
        jti: Uuid::new_v4(),
    };

    let key = EncodingKey::from_secret(secret.as_bytes());
    let access_token = encode(&Header::default(), &access_claims, &key)?;
    let refresh_token = encode(&Header::default(), &refresh_claims, &key)?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_at: access_exp,
    })
}

/// Verify and decode a JWT token.
pub fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    let data = decode::<Claims>(token, &key, &validation)?;
    Ok(data.claims)
}
