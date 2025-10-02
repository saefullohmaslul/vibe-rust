use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};



#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/health", get(health));

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
