use std::sync::Arc;
use uuid::Uuid;

use crate::models::model::{NoteModel, NoteModelResponse};
use super::{
    repository::NoteRepository,
    CreateNoteSchema, UpdateNoteSchema, FilterOptions,
};

pub struct NoteService {
    repository: Arc<NoteRepository>,
}

impl NoteService {
    pub fn new(repository: Arc<NoteRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_notes(&self, opts: FilterOptions) -> Result<Vec<NoteModelResponse>, String> {
        let limit = opts.limit.unwrap_or(10) as i32;
        let page = opts.page.unwrap_or(1);
        let offset = (page - 1) * limit as usize;

        let notes = self
            .repository
            .get_all_notes(limit, offset as i32)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(notes.iter().map(|note| self.to_note_response(note)).collect())
    }

    pub async fn create_note(&self, note_data: CreateNoteSchema) -> Result<NoteModelResponse, String> {
        let id = Uuid::new_v4().to_string();
        let is_published = note_data.is_published.unwrap_or(false);

        let note = self
            .repository
            .create_note(&id, &note_data.title, &note_data.content, is_published)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(self.to_note_response(&note))
    }

    pub async fn update_note(
        &self,
        id: String,
        note_data: UpdateNoteSchema,
    ) -> Result<NoteModelResponse, String> {
        let _uuid = Uuid::parse_str(&id)
            .map_err(|e| format!("Invalid UUID format: {}", e))?;

        let existing_note = self
            .repository
            .get_by_id(&id)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let title = note_data.title.unwrap_or_else(|| existing_note.title);
        let content = note_data.content.unwrap_or_else(|| existing_note.content);
        let is_published = note_data.is_published.unwrap_or(existing_note.is_published);

        let updated_note = self
            .repository
            .update_note(&id, &title, &content, is_published)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(self.to_note_response(&updated_note))
    }

    fn to_note_response(&self, note: &NoteModel) -> NoteModelResponse {
        NoteModelResponse {
            id: note.id.clone(),
            title: note.title.clone(),
            content: note.content.clone(),
            is_published: note.is_published,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}