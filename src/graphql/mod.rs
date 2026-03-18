pub mod analytics;
pub mod clusters;
pub mod dashboard;
pub mod finops;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::AppState;

/// Combined query root merging all domain queries.
#[derive(MergedObject, Default)]
pub struct QueryRoot(
    dashboard::DashboardQuery,
    finops::FinOpsQuery,
    clusters::ClustersQuery,
    analytics::AnalyticsQuery,
);

/// Combined mutation root.
#[derive(MergedObject, Default)]
pub struct MutationRoot(
    analytics::AnalyticsMutation,
);

pub type MetatronSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build the async-graphql schema with shared state.
pub fn build_schema(state: AppState) -> MetatronSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state)
    .finish()
}
