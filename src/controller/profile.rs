use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};

use crate::{
    model::{ProfileDto, User, UserWithGroups},
    repository::UserRepository,
    security::Jwt,
};

use super::Errors;

pub fn routes(repo: UserRepository) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", put(update))
        .with_state(repo)
}

pub async fn index(
    State(repo): State<UserRepository>,
    jwt: Jwt,
) -> Result<Json<UserWithGroups>, (StatusCode, Json<Errors>)> {
    repo.find_with_groups(jwt.id)
        .await
        .map(Json)
        .map_err(Errors::sql)
}

pub async fn update(
    State(repo): State<UserRepository>,
    jwt: Jwt,
    Json(body): Json<ProfileDto>,
) -> Result<Json<User>, (StatusCode, Json<Errors>)> {
    repo.update_profile(jwt.id, body)
        .await
        .map(Json)
        .map_err(Errors::sql)
}
