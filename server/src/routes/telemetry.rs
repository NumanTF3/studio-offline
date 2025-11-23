use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::app_state::AppState;
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/game/validate-machine", get(validate_machine))
        .route("/validate-machine", post(validate_machine))
        .route("/studio/pbe", get(studio_pbe))
        .route("/studio/pbe", post(studio_pbe))
        .route(
            "/v1.0/SequenceStatistics/BatchAddToSequencesV2",
            get(sequence_statistics),
        )
}

async fn validate_machine() -> impl IntoResponse {
    Json(json!({ "success": true, "message": "" }))
}

async fn studio_pbe() -> impl IntoResponse {
    Json(json!({}))
}

async fn sequence_statistics() -> impl IntoResponse {
    Json(json!({
        "Version": "1.1",
        "Content": { "Headers": [] },
        "StatusCode": "OK",
        "ReasonPhrase": "OK",
        "Headers": [],
        "TrailingHeaders": [],
        "RequestMessage": null,
        "IsSuccessStatusCode": true
    }))
}
