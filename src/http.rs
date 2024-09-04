use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use ulid::Ulid;

use crate::{domains::tasks, AppState};

pub async fn tasks_get(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let task = state
        .tasks_repo
        .load(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(task) = task {
        return Ok(Json(task));
    }

    Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
}

pub async fn tasks_create(
    State(state): State<AppState>,
    Json(input): Json<tasks::inputs::Create>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let command_id = Ulid::new().to_string();
    let aggregate_id = Ulid::new().to_string();

    let mut metadata = HashMap::<String, String>::new();
    metadata.insert("command_id".to_string(), command_id);

    let command = tasks::Command::Create {
        id: aggregate_id.clone(),
        input,
    };

    state
        .tasks_cqrs
        .execute_with_metadata(&aggregate_id, command, metadata)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Now that the command is committed, retrieve the result from the view
    let task = state
        .tasks_repo
        .load(&aggregate_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(task) = task {
        return Ok((StatusCode::CREATED, Json(task)));
    }

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Task was not found after creation".to_string(),
    ))
}

pub async fn tasks_update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<tasks::inputs::Update>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let command_id = Ulid::new().to_string();

    let mut metadata = HashMap::<String, String>::new();
    metadata.insert("command_id".to_string(), command_id);

    let command = tasks::Command::Update(input);

    state
        .tasks_cqrs
        .execute_with_metadata(&id, command, metadata)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Now that the command is committed, retrieve the result from the view
    let task = state
        .tasks_repo
        .load(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(task) = task {
        return Ok(Json(task));
    }

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Task was not found after update".to_string(),
    ))
}

pub async fn tasks_delete(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let command_id = Ulid::new().to_string();

    let mut metadata = HashMap::<String, String>::new();
    metadata.insert("command_id".to_string(), command_id);

    let command = tasks::Command::Delete;

    state
        .tasks_cqrs
        .execute_with_metadata(&id, command, metadata)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, "Task deleted".to_string()))
}
