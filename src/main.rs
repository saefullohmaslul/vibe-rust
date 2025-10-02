use axum::{response::IntoResponse, routing::get, Json, Router};



#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    println!("Server is running on port 8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn health() -> impl IntoResponse {
    const MESSAGE: &str = "API is healthy";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE,
    });

    Json(json_response)
}
