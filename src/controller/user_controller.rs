use crate::{
    model::{PasswordDto, UserUpdateDto},
    security::{password, Jwt},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    model::{UserCreateDto, UserWithGroups},
    repository::UserRepository,
};

use super::Errors;

pub fn routes(repo: UserRepository) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/:id", get(show))
        .route("/", post(create))
        .route("/:id", put(update))
        .route("/:id/password", put(update_password))
        .with_state(repo)
}

#[axum::debug_handler]
pub async fn index(
    State(repo): State<UserRepository>,
    jwt: Jwt,
) -> Result<Json<Vec<UserWithGroups>>, (StatusCode, Json<Errors>)> {
    if !jwt.has_permission("user:read") {
        return Err(Errors::forbidden());
    }

    repo.find_all_with_groups()
        .await
        .map(Json)
        .map_err(Errors::sql)
}

#[axum::debug_handler]
pub async fn show(
    jwt: Jwt,
    State(repo): State<UserRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserWithGroups>, (StatusCode, Json<Errors>)> {
    if !jwt.has_permission("user:read") {
        return Err(Errors::forbidden());
    }

    repo.find_with_groups(id)
        .await
        .map(Json)
        .map_err(Errors::sql)
}

#[axum::debug_handler]
pub async fn create(
    jwt: Jwt,
    State(repo): State<UserRepository>,
    Json(dto): Json<UserCreateDto>,
) -> Result<Json<UserWithGroups>, (StatusCode, Json<Errors>)> {
    if !jwt.has_permission("user:create") {
        return Err(Errors::forbidden());
    }

    let password_hash = password::hash(&dto.password).map_err(Errors::argon2)?;
    let (visible, editable) = if jwt.is_root() {
        (dto.visible, dto.editable)
    } else {
        (true, true)
    };
    let locked = if jwt.is_admin() { dto.locked } else { false };

    let dto = UserCreateDto {
        password_hash,
        visible,
        editable,
        locked,
        ..dto
    };
    repo.create(dto).await.map(Json).map_err(Errors::sql)
}

#[axum::debug_handler]
pub async fn update(
    State(repo): State<UserRepository>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UserUpdateDto>,
) -> Result<Json<UserWithGroups>, (StatusCode, String)> {
    match repo.update(id, dto).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn update_password(
    State(repo): State<UserRepository>,
    Path(id): Path<Uuid>,
    Json(dto): Json<PasswordDto>,
) -> Result<StatusCode, (StatusCode, String)> {
    // hash plain password with argon2
    let hash = match password::hash(&dto.password) {
        Ok(hash) => hash,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    let dto = PasswordDto {
        password_hash: hash,
        ..dto
    };

    match repo.update_password(id, dto).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
