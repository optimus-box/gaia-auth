use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::Group;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password_hash: Vec<u8>,
    pub visible: bool,
    pub editable: bool,
    pub locked: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UserWithGroups {
    #[serde(flatten)]
    pub user: User,
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize)]
pub struct UserCreateDto {
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub visible: bool,
    pub editable: bool,
    pub locked: bool,
    #[serde(skip)]
    pub password_hash: Vec<u8>,
    pub groups: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdateDto {
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub username: String,
    pub visible: bool,
    pub editable: bool,
    pub locked: bool,
    pub groups: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileDto {
    pub name: String,
    pub phone: Option<String>,
    pub role: Option<String>,
}
