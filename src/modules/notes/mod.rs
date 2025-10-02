use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod handler;
pub mod service;
pub mod routes;
pub mod repository;

pub use service::NoteService;
pub use repository::NoteRepository;

#[derive(Deserialize, Debug, Default, ToSchema)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

pub struct AppState {
    pub note_service: Arc<NoteService>,
}