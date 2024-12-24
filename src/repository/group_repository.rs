use crate::model::{Group, GroupDto};
use sqlx::{query_as, query_scalar, types::Json, Pool};
use uuid::Uuid;

#[derive(Clone)]
pub struct GroupRepository {
    db: Pool<sqlx::Postgres>,
}

impl GroupRepository {
    pub fn new(db: Pool<sqlx::Postgres>) -> GroupRepository {
        GroupRepository { db }
    }

    fn db(&self) -> &Pool<sqlx::Postgres> {
        &self.db
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let sql = "select count(*) from groups";
        query_scalar(sql).fetch_one(self.db()).await
    }

    pub async fn find_all(&self) -> Result<Vec<Group>, sqlx::Error> {
        let sql = "select * from groups";
        query_as(sql).fetch_all(self.db()).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Group, sqlx::Error> {
        let sql = "select * from groups where id = $1";
        query_as(sql).bind(id).fetch_one(self.db()).await
    }

    pub async fn create(&self, dto: GroupDto) -> Result<Group, sqlx::Error> {
        let sql = r#"insert into groups
            (name, description, permissions, visible, editable, locked)
        values
            ($1, $2, $3, $4, $5, $6)
        returning *"#;
        query_as(&sql)
            .bind(dto.name)
            .bind(dto.description)
            .bind(Json(dto.permissions))
            .bind(dto.visible.unwrap_or(true))
            .bind(dto.editable.unwrap_or(true))
            .bind(dto.locked.unwrap_or(false))
            .fetch_one(self.db())
            .await
    }

    pub async fn update(&self, id: Uuid, dto: GroupDto) -> Result<Group, sqlx::Error> {
        let sql = r#"update groups set
            name = $2,
            description = $3,
            permissions = $4,
            visible = $5,
            editable = $6,
            locked = $7
        where id = $1 returning *"#;
        query_as(&sql)
            .bind(id)
            .bind(dto.name)
            .bind(dto.description)
            .bind(Json(dto.permissions))
            .bind(dto.visible.unwrap_or(true))
            .bind(dto.editable.unwrap_or(true))
            .bind(dto.locked.unwrap_or(false))
            .fetch_one(self.db())
            .await
    }
}
