use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Json<Vec<String>>,
    pub visible: bool,
    pub editable: bool,
    pub locked: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GroupDto {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub visible: Option<bool>,
    pub editable: Option<bool>,
    pub locked: Option<bool>,
}

impl Group {
    pub fn permissions(&self) -> Vec<String> {
        let mut list = Vec::new();
        for permission in self.permissions.iter() {
            list.push(permission.clone());
        }
        list
    }
}
