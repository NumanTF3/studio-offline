use axum::{Router, response::IntoResponse, routing::get};
use tower_http::services::ServeFile;

use crate::app_state::AppState;
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route_service(
            "/universal-app-configuration/v1/behaviors/studio/content",
            ServeFile::new("static/config/UniversalAppConfig/content.json"),
        )
        .route_service(
            "/universal-app-configuration/v1/behavior-contents",
            ServeFile::new("static/config/UniversalAppConfig/behavior-content.json"),
        )
        .route("/guac-v2/v1/bundles/studio", get(bundles_studio))
}

async fn bundles_studio() -> impl IntoResponse {
    axum::Json(serde_json::json!({}))
}
