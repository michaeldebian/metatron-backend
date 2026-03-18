use async_graphql::{Context, Object, SimpleObject};

use crate::AppState;

#[derive(Default)]
pub struct DashboardQuery;

#[derive(SimpleObject)]
pub struct WidgetData {
    pub id: String,
    pub widget_type: String,
    pub title: String,
    pub data_json: String,
}

#[Object]
impl DashboardQuery {
    /// Get dashboard widget data for the current user's persona/permissions.
    async fn dashboard_widgets(
        &self,
        ctx: &Context<'_>,
        persona: Option<String>,
    ) -> async_graphql::Result<Vec<WidgetData>> {
        let _state = ctx.data::<AppState>()?;

        // TODO: Query actual dashboard data based on persona/permissions
        Ok(vec![
            WidgetData {
                id: "fleet-health".to_string(),
                widget_type: "kpi".to_string(),
                title: "Fleet Health".to_string(),
                data_json: r#"{"value": 98.5, "unit": "%", "trend": "up"}"#.to_string(),
            },
            WidgetData {
                id: "active-services".to_string(),
                widget_type: "kpi".to_string(),
                title: "Active Services".to_string(),
                data_json: r#"{"value": 47, "unit": "", "trend": "stable"}"#.to_string(),
            },
        ])
    }
}
