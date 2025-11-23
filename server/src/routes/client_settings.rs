use axum::Router;

use tower_http::services::ServeFile;

pub fn routes() -> Router<std::sync::Arc<crate::app_state::AppState>> {
    Router::new().route_service(
        "/",
        ServeFile::new("static/config/ClientSettings/PCStudioApp.json"),
    )
}
