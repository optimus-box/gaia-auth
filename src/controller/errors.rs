use axum::{http::StatusCode, Json};

#[derive(serde::Serialize)]
pub struct Errors {
    error: String,
}

impl Errors {
    pub fn unauthorized(err: &str) -> (StatusCode, Json<Errors>) {
        (
            StatusCode::UNAUTHORIZED,
            Json(Errors {
                error: String::from(err),
            }),
        )
    }

    pub fn not_found() -> (StatusCode, Json<Errors>) {
        (
            StatusCode::NOT_FOUND,
            Json(Errors {
                error: String::from("object not found"),
            }),
        )
    }

    pub fn forbidden() -> (StatusCode, Json<Errors>) {
        (
            StatusCode::FORBIDDEN,
            Json(Errors {
                error: String::from("you have not permission for acess this content"),
            }),
        )
    }

    pub fn internal(err: &str) -> (StatusCode, Json<Errors>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Errors {
                error: String::from(err),
            }),
        )
    }

    pub fn sql(err: sqlx::Error) -> (StatusCode, Json<Errors>) {
        match err {
            sqlx::Error::RowNotFound => Self::not_found(),
            _ => Self::internal(&err.to_string()),
        }
    }

    pub fn argon2(err: argon2::Error) -> (StatusCode, Json<Errors>) {
        Self::internal(&err.to_string())
    }
}
