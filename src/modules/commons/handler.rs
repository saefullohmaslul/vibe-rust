use axum::Json;
use axum::response::IntoResponse;

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
