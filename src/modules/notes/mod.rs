use std::sync::Arc;

use serde::{Deserialize, Serialize};
use shaku::module;
use utoipa::ToSchema;

pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::{NoteRepositoryImpl, NoteRepositoryImplParameters};
pub use service::NoteService;

module! {
    pub NotesModule {
        components = [repository::NoteRepositoryImpl, service::NoteServiceImpl],
        providers = []
    }
}

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
    pub note_service: Arc<dyn NoteService>,
}
