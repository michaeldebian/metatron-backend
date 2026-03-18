use async_graphql::{Context, InputObject, Object, SimpleObject};

use crate::AppState;

#[derive(Default)]
pub struct AnalyticsQuery;

#[derive(Default)]
pub struct AnalyticsMutation;

#[derive(SimpleObject)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows_json: Vec<String>,
    pub rows_read: i64,
    pub timing_ms: f64,
    pub error: Option<String>,
}

#[derive(SimpleObject)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(SimpleObject)]
pub struct DatabaseInfo {
    pub name: String,
}

#[derive(SimpleObject)]
pub struct TableInfo {
    pub name: String,
    pub engine: String,
    pub row_count: i64,
}

#[derive(InputObject)]
pub struct SqlQueryInput {
    pub sql: String,
    pub database: Option<String>,
    pub warehouse: Option<String>,
    pub max_rows: Option<i32>,
}

#[Object]
impl AnalyticsQuery {
    /// List databases available in Databend.
    async fn databases(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<DatabaseInfo>> {
        let state = ctx.data::<AppState>()?;
        let dbs = state.databend.list_databases().await?;
        Ok(dbs
            .into_iter()
            .map(|name| DatabaseInfo { name })
            .collect())
    }

    /// List tables in a specific database.
    async fn tables(
        &self,
        ctx: &Context<'_>,
        database: String,
    ) -> async_graphql::Result<Vec<TableInfo>> {
        let state = ctx.data::<AppState>()?;
        let tables = state.databend.list_tables(&database).await?;
        Ok(tables
            .into_iter()
            .map(|(name, engine, row_count)| TableInfo {
                name,
                engine,
                row_count,
            })
            .collect())
    }
}

#[Object]
impl AnalyticsMutation {
    /// Execute a SQL query against Databend.
    async fn execute_sql(
        &self,
        ctx: &Context<'_>,
        input: SqlQueryInput,
    ) -> async_graphql::Result<QueryResult> {
        let state = ctx.data::<AppState>()?;

        match state
            .databend
            .query(&input.sql, input.max_rows.unwrap_or(1000))
            .await
        {
            Ok((columns, rows, rows_read, timing_ms)) => Ok(QueryResult {
                columns: columns
                    .into_iter()
                    .map(|(name, data_type)| ColumnInfo { name, data_type })
                    .collect(),
                rows_json: rows,
                rows_read,
                timing_ms,
                error: None,
            }),
            Err(e) => Ok(QueryResult {
                columns: vec![],
                rows_json: vec![],
                rows_read: 0,
                timing_ms: 0.0,
                error: Some(e.to_string()),
            }),
        }
    }
}
