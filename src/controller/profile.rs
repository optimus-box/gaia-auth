use axum::{extract::State, routing::get, Json, Router};

use crate::{model::UserWithGroups, repository::UserRepository, security::Jwt};

pub fn routes(repo: UserRepository) -> Router {
    Router::new().route("/", get(index)).with_state(repo)
}

pub async fn index(State(repo): State<UserRepository>, jwt: Jwt) -> Json<UserWithGroups> {
    let user = repo.find_with_groups(jwt.id).await.unwrap();
    Json(user)
}
