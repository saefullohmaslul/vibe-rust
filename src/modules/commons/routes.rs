use axum::{routing::get, Router};

use super::handler::health;

pub fn create_commons_router() -> Router {
    Router::new()
        .route("/health", get(health))
}