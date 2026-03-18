use async_graphql::{Context, Object, SimpleObject};

use crate::AppState;

#[derive(Default)]
pub struct FinOpsQuery;

#[derive(SimpleObject)]
pub struct SpendSummary {
    pub total_cost_usd: f64,
    pub previous_period_cost_usd: f64,
    pub change_percent: f64,
    pub currency: String,
    pub breakdown: Vec<CostDimension>,
}

#[derive(SimpleObject)]
pub struct CostDimension {
    pub key: String,
    pub cost_usd: f64,
    pub change_percent: f64,
}

#[derive(SimpleObject)]
pub struct CommitmentMetrics {
    pub esr: f64,
    pub coverage_rate: f64,
    pub utilization_rate: f64,
    pub total_savings_monthly_usd: f64,
    pub on_demand_percent: f64,
}

#[Object]
impl FinOpsQuery {
    /// Get cost spend summary for a date range, optionally filtered by provider.
    async fn spend_summary(
        &self,
        ctx: &Context<'_>,
        provider: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> async_graphql::Result<SpendSummary> {
        let state = ctx.data::<AppState>()?;

        // TODO: Call FinOpsService gRPC client
        // For now return stub data matching admin1's finops-mock.ts
        let _ = (state, provider, start_date, end_date);

        Ok(SpendSummary {
            total_cost_usd: 127_450.0,
            previous_period_cost_usd: 134_200.0,
            change_percent: -5.03,
            currency: "USD".to_string(),
            breakdown: vec![
                CostDimension {
                    key: "Compute".to_string(),
                    cost_usd: 62_300.0,
                    change_percent: -3.2,
                },
                CostDimension {
                    key: "Storage".to_string(),
                    cost_usd: 28_100.0,
                    change_percent: 1.5,
                },
                CostDimension {
                    key: "Network".to_string(),
                    cost_usd: 18_900.0,
                    change_percent: -8.1,
                },
                CostDimension {
                    key: "Database".to_string(),
                    cost_usd: 18_150.0,
                    change_percent: -6.4,
                },
            ],
        })
    }

    /// Get commitment coverage metrics (RI/SP/CUD).
    async fn commitment_metrics(
        &self,
        ctx: &Context<'_>,
        provider: Option<String>,
    ) -> async_graphql::Result<CommitmentMetrics> {
        let _state = ctx.data::<AppState>()?;
        let _ = provider;

        Ok(CommitmentMetrics {
            esr: 31.2,
            coverage_rate: 68.5,
            utilization_rate: 92.3,
            total_savings_monthly_usd: 41_200.0,
            on_demand_percent: 31.5,
        })
    }
}
