use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};

use super::{
    AppState,
    handler::{create_note_handler, get_list_note_handler, update_note_handler},
};

pub fn create_notes_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/notes", get(get_list_note_handler))
        .route("/notes", post(create_note_handler))
        .route("/notes/{id}", put(update_note_handler))
        .with_state(app_state)
}
