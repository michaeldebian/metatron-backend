/// Database module — PostgreSQL pool and migration support.

use sqlx::PgPool;
use uuid::Uuid;

/// Create a new organization and its owner user. Returns (org_id, user_id).
pub async fn create_org_with_owner(
    pool: &PgPool,
    org_name: &str,
    org_slug: &str,
    email: &str,
    name: &str,
    password_hash: &str,
) -> anyhow::Result<(Uuid, Uuid)> {
    let mut tx = pool.begin().await?;

    let org_row: (Uuid,) = sqlx::query_as(
        "INSERT INTO organizations (name, slug) VALUES ($1, $2) RETURNING id",
    )
    .bind(org_name)
    .bind(org_slug)
    .fetch_one(&mut *tx)
    .await?;
    let org_id = org_row.0;

    let user_row: (Uuid,) = sqlx::query_as(
        "INSERT INTO users (org_id, email, name, password_hash, is_org_owner) VALUES ($1, $2, $3, $4, true) RETURNING id",
    )
    .bind(org_id)
    .bind(email)
    .bind(name)
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await?;
    let user_id = user_row.0;

    tx.commit().await?;
    Ok((org_id, user_id))
}

/// Find a user by email.
pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> anyhow::Result<Option<UserRow>> {
    let row: Option<UserRow> = sqlx::query_as(
        r#"
        SELECT id, org_id, email, name, avatar_url, password_hash,
               identity_provider, is_org_owner, last_login_at, created_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Update last_login_at timestamp.
pub async fn touch_login(pool: &PgPool, user_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET last_login_at = now() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub password_hash: Option<String>,
    pub identity_provider: String,
    pub is_org_owner: bool,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
