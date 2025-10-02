use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::{AppState, CreateNoteSchema, FilterOptions, UpdateNoteSchema};

#[utoipa::path(
    get,
    path = "/api/v1/notes",
    tag = "Notes",
    params(
        ("limit" = Option<i32>, Query, description = "Limit number of notes returned"),
        ("page" = Option<i32>, Query, description = "Page number for pagination")
    ),
    responses(
        (status = 200, description = "Notes retrieved successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn get_list_note_handler(
    Query(opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note_service = data.note_service.clone();

    let notes = note_service.get_notes(opts).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": e,
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "OK",
        "message": "Notes retrieved successfully",
        "data": notes,
    });

    Ok(Json(json_response))
}

#[utoipa::path(
    post,
    path = "/api/v1/notes",
    tag = "Notes",
    request_body = CreateNoteSchema,
    responses(
        (status = 200, description = "Note created successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(note): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note_service = data.note_service.clone();

    let created_note = note_service.create_note(note).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": e,
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "OK",
        "message": "Note created successfully",
        "data": created_note,
    });

    Ok(Json(json_response))
}

#[utoipa::path(
    put,
    path = "/api/v1/notes/{id}",
    tag = "Notes",
    params(
        ("id" = String, Path, description = "Note ID to update")
    ),
    request_body = UpdateNoteSchema,
    responses(
        (status = 200, description = "Note updated successfully", body = serde_json::Value),
        (status = 400, description = "Invalid UUID format", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn update_note_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(note): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note_service = data.note_service.clone();

    let updated_note = note_service.update_note(id, note).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": e,
        });

        if e.contains("Invalid UUID format") {
            (StatusCode::BAD_REQUEST, Json(error_response))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    })?;

    let json_response = serde_json::json!({
        "status": "OK",
        "message": "Note updated successfully",
        "data": updated_note,
    });

    Ok(Json(json_response))
}
