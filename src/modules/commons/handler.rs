use axum::{Json, Router, response::IntoResponse, routing::get};

pub fn create_commons_router() -> Router {
    Router::new().route("/health", get(health))
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Common",
    responses(
        (status = 200, description = "API Health Check", body = serde_json::Value)
    )
)]
pub async fn health() -> impl IntoResponse {
    const MESSAGE: &str = "API is healthy";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE,
    });

    Json(json_response)
}
