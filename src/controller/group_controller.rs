use crate::{
    model::{Group, GroupDto},
    repository::GroupRepository,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use uuid::Uuid;

pub fn routes(repo: GroupRepository) -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .route("/:id", axum::routing::get(show))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::put(update))
        .with_state(repo)
}

#[axum::debug_handler]
pub async fn index(
    State(repo): State<GroupRepository>,
) -> Result<Json<Vec<Group>>, (StatusCode, String)> {
    match repo.find_all().await {
        Ok(groups) => Ok(Json(groups)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn show(
    State(repo): State<GroupRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<Group>, (StatusCode, String)> {
    match repo.find_by_id(id).await {
        Ok(group) => Ok(Json(group)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn create(
    State(repo): State<GroupRepository>,
    Json(dto): Json<GroupDto>,
) -> Result<Json<Group>, (StatusCode, String)> {
    match repo.create(dto).await {
        Ok(group) => Ok(Json(group)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn update(
    State(repo): State<GroupRepository>,
    Path(id): Path<Uuid>,
    Json(dto): Json<GroupDto>,
) -> Result<Json<Group>, (StatusCode, String)> {
    match repo.update(id, dto).await {
        Ok(group) => Ok(Json(group)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
