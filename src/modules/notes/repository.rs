use std::sync::Arc;
use sqlx::PgPool;

use crate::models::model::NoteModel;

pub struct NoteRepository {
    pool: Arc<PgPool>,
}

impl NoteRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_all_notes(&self, limit: i32, offset: i32) -> Result<Vec<NoteModel>, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(
            "SELECT id, title, content, is_published, created_at, updated_at FROM notes LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn create_note(
        &self,
        id: &str,
        title: &str,
        content: &str,
        is_published: bool,
    ) -> Result<NoteModel, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(
            "
        INSERT INTO notes (
            id,
            title,
            content,
            is_published
        ) VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            title,
            content,
            is_published,
            created_at,
            updated_at
        ",
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(is_published)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<NoteModel, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(
            "SELECT id, title, content, is_published, created_at, updated_at FROM notes WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_note(
        &self,
        id: &str,
        title: &str,
        content: &str,
        is_published: bool,
    ) -> Result<NoteModel, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(
            "
        UPDATE notes
        SET
            title = $2,
            content = $3,
            is_published = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            title,
            content,
            is_published,
            created_at,
            updated_at
        ",
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(is_published)
        .fetch_one(&*self.pool)
        .await
    }
}