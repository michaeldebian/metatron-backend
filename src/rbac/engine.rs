use std::collections::{HashMap, HashSet};

use sqlx::PgPool;
use uuid::Uuid;

use super::types::{ResolvedPermissions, SYSTEM_PERMISSION_SETS};

/// Resolve all effective permissions for a user by combining:
/// 1. All permission set grants from teams the user belongs to (UNION)
/// 2. User-level overrides (grant=true adds, grant=false removes)
pub async fn resolve_user_permissions(
    pool: &PgPool,
    user_id: Uuid,
    org_id: Uuid,
    is_owner: bool,
) -> anyhow::Result<ResolvedPermissions> {
    // Org owners get everything
    if is_owner {
        let mut grants = HashMap::new();
        for module in super::types::MODULES {
            grants.insert(
                module.to_string(),
                super::types::ACTIONS.iter().map(|a| a.to_string()).collect(),
            );
        }
        return Ok(ResolvedPermissions {
            grants,
            team_ids: get_user_team_ids(pool, user_id).await?,
            credential_ids: get_user_credential_ids(pool, user_id).await?,
        });
    }

    // Step 1: Collect grants from all team permission sets
    let rows: Vec<(sqlx::types::Json<Vec<GrantRow>>,)> = sqlx::query_as(
        r#"
        SELECT ps.grants
        FROM permission_sets ps
        JOIN team_permission_sets tps ON tps.permission_set_id = ps.id
        JOIN team_members tm ON tm.team_id = tps.team_id
        WHERE tm.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut grants: HashMap<String, HashSet<String>> = HashMap::new();
    for (json_grants,) in &rows {
        for grant in json_grants.iter() {
            grants
                .entry(grant.module.clone())
                .or_default()
                .insert(grant.action.clone());
        }
    }

    // Step 2: Apply user-level overrides
    let overrides: Vec<(String, String, bool)> = sqlx::query_as(
        "SELECT module, action, granted FROM user_permission_overrides WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    for (module, action, granted) in &overrides {
        if *granted {
            grants
                .entry(module.clone())
                .or_default()
                .insert(action.clone());
        } else {
            if let Some(actions) = grants.get_mut(module) {
                actions.remove(action);
            }
        }
    }

    Ok(ResolvedPermissions {
        grants,
        team_ids: get_user_team_ids(pool, user_id).await?,
        credential_ids: get_user_credential_ids(pool, user_id).await?,
    })
}

/// Seed the built-in system permission sets on startup.
pub async fn seed_system_permission_sets(pool: &PgPool) -> anyhow::Result<()> {
    for ps in SYSTEM_PERMISSION_SETS {
        let grants_json: Vec<GrantRow> = ps
            .grants
            .iter()
            .flat_map(|(module, actions)| {
                actions.iter().map(move |action| GrantRow {
                    module: module.to_string(),
                    action: action.to_string(),
                })
            })
            .collect();

        let grants = serde_json::to_value(&grants_json)?;

        sqlx::query(
            r#"
            INSERT INTO permission_sets (name, description, is_system, grants, org_id)
            VALUES ($1, $2, true, $3, NULL)
            ON CONFLICT (name) WHERE org_id IS NULL AND is_system = true
            DO UPDATE SET grants = $3, description = $2
            "#,
        )
        .bind(ps.name)
        .bind(ps.description)
        .bind(&grants)
        .execute(pool)
        .await
        .ok(); // Ignore if constraint doesn't exist during initial migration
    }
    Ok(())
}

async fn get_user_team_ids(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<Uuid>> {
    let rows: Vec<(Uuid,)> =
        sqlx::query_as("SELECT team_id FROM team_members WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

async fn get_user_credential_ids(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<Uuid>> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT ctg.credential_id
        FROM credential_team_grants ctg
        JOIN team_members tm ON tm.team_id = ctg.team_id
        WHERE tm.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct GrantRow {
    module: String,
    action: String,
}
