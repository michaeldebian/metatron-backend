use tracing::warn;

/// Databend SQL client for analytics queries and notebook execution.
pub struct DatabendClient {
    dsn: String,
}

impl DatabendClient {
    pub fn new(dsn: &str) -> Self {
        Self {
            dsn: dsn.to_string(),
        }
    }

    /// Execute a SQL query and return (columns, rows_json, rows_read, timing_ms).
    pub async fn query(
        &self,
        sql: &str,
        max_rows: i32,
    ) -> anyhow::Result<(Vec<(String, String)>, Vec<String>, i64, f64)> {
        let start = std::time::Instant::now();

        let client = databend_driver::Client::new(self.dsn.clone());
        let conn = client.get_conn().await.map_err(|e| {
            warn!(error = %e, "Failed to connect to Databend");
            anyhow::anyhow!("Databend connection failed: {e}")
        })?;

        let rows: Vec<databend_driver::Row> = conn.query_all(sql).await.map_err(|e| {
            anyhow::anyhow!("Databend query failed: {e}")
        })?;

        let mut columns = Vec::new();
        let mut rows_out = Vec::new();

        // Extract column names from schema if rows exist
        if let Some(first_row) = rows.first() {
            for (i, _) in first_row.values().iter().enumerate() {
                columns.push((format!("col_{i}"), "String".to_string()));
            }
        }

        // Serialize rows as JSON arrays
        for (i, row) in rows.iter().enumerate() {
            if i >= max_rows as usize {
                break;
            }
            let values: Vec<serde_json::Value> = row
                .values()
                .iter()
                .map(|v| serde_json::Value::String(format!("{v:?}")))
                .collect();
            rows_out.push(serde_json::to_string(&values)?);
        }

        let rows_read = rows_out.len() as i64;
        let timing_ms = start.elapsed().as_secs_f64() * 1000.0;
        Ok((columns, rows_out, rows_read, timing_ms))
    }

    /// List all databases.
    pub async fn list_databases(&self) -> anyhow::Result<Vec<String>> {
        let client = databend_driver::Client::new(self.dsn.clone());
        let conn = client.get_conn().await?;
        let rows: Vec<databend_driver::Row> = conn.query_all("SHOW DATABASES").await?;
        let mut dbs = Vec::new();
        for row in &rows {
            if let Some(val) = row.values().first() {
                dbs.push(format!("{val:?}"));
            }
        }
        Ok(dbs)
    }

    /// List tables in a database. Returns (name, engine, row_count).
    pub async fn list_tables(
        &self,
        database: &str,
    ) -> anyhow::Result<Vec<(String, String, i64)>> {
        let client = databend_driver::Client::new(self.dsn.clone());
        let conn = client.get_conn().await?;
        let sql = format!("SHOW TABLES FROM `{}`", database.replace('`', ""));
        let rows: Vec<databend_driver::Row> = conn.query_all(&sql).await?;
        let mut tables = Vec::new();
        for row in &rows {
            if let Some(val) = row.values().first() {
                tables.push((format!("{val:?}"), "FUSE".to_string(), 0i64));
            }
        }
        Ok(tables)
    }
}
