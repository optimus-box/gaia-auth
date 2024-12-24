use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Serialize;

use crate::{
    model::{LoginDto, UserWithGroups},
    repository::UserRepository,
    security::{self, password},
};

use super::Errors;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(flatten)]
    pub user: UserWithGroups,
    pub token: String,
}

pub fn router(repo: UserRepository) -> Router {
    Router::new().route("/login", post(login)).with_state(repo)
}

/// Authenticates a user using a username and password.
///
/// # Errors
///
/// * `unauthorized` - if the username or password is incorrect
/// * `internal_error` - if there was a problem with the database or password hashing
#[axum::debug_handler]
pub async fn login(
    State(repo): State<UserRepository>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<Errors>)> {
    let user = match repo.find_by_username(dto.username).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return Err(Errors::unauthorized("username or password is incorrect"));
        }
        Err(err) => return Err(Errors::sql(err)),
    };

    match password::check(&user.user.password_hash, &dto.password) {
        Ok(checked) => {
            if checked {
                match security::jwt::generate_token(&user) {
                    Ok(token) => return Ok(Json(LoginResponse { user, token })),
                    Err(err) => return Err(err),
                }
            }
            Err(Errors::unauthorized("username or password is incorrect"))
        }
        Err(err) => Err(Errors::argon2(err)),
    }
}
