//! Implements all CRUD functionalities.

use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::NoteModel;
use crate::schema::UpdateNoteSchema;
use crate::{
    schema::{CreateNoteSchema, FilterOptions},
    AppState,
};

/// Axum Route Handler to check server status.
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Building a simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

/// Axum Route Handler to Add a Record.
pub async fn create_note_handler(
    State(data): State<Arc<RwLock<AppState>>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.write().await.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({ "status": "success", "data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(err) => {
            if err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response =
                    json!({"status": "fail", "message": "Note with that title already exists"});

                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", err)})),
            ));
        }
    }
}

/// Axum Route Handler to Fetch All Records.
pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Query(options) = opts.unwrap_or_default();

    let limit = options.limit.unwrap_or(10);
    let offset = (options.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.read().await.db)
    .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items"
        });

        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });

    Ok(Json(json_response))
}

/// Axum Route Handler to Fetch a Record.
pub async fn get_note_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.read().await.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({
                "status": "success",
                "data": json!({
                    "note": note
                })
            });

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });

            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

/// Axum Route Handler to Edit a Record.
pub async fn edit_note_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.read().await.db)
        .await;

    if query_result.is_err() {
        let error_response = json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });

        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.to_owned().unwrap_or(note.published.unwrap()),
        now,
        id
    ).fetch_one(&data.write().await.db).await;

    match query_result {
        Ok(note) => {
            let note_response = json!({
                "status": "success",
                "data": json!({
                    "note": note
                })
            });

            return Ok(Json(note_response));
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": format!("{:?}", err)
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    }
}

/// Axum Route Handler to Delete a Record.
pub async fn delete_note_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(&data.write().await.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });

        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
