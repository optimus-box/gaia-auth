use sqlx::{query, query_as, Pool, Postgres};
use uuid::Uuid;

use crate::model::{Group, PasswordDto, User, UserCreateDto, UserUpdateDto, UserWithGroups};

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<sqlx::Postgres>,
}

impl UserRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        UserRepository { db }
    }

    fn db(&self) -> &Pool<Postgres> {
        &self.db
    }

    async fn unsign(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        let sql = "delete from users_groups where user_id = $1";
        match query(sql).bind(user_id).execute(self.db()).await {
            Ok(_) => Ok(()),
            Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn assign(&self, user_id: Uuid, groups: Vec<Uuid>) -> Result<(), sqlx::Error> {
        self.unsign(user_id).await?;
        let sql = "insert into users_groups (user_id, group_id) select $1, unnest($2)";
        query(sql)
            .bind(user_id)
            .bind(groups)
            .execute(self.db())
            .await?;
        Ok(())
    }

    async fn groups(&self, user_id: Uuid) -> Result<Vec<Group>, sqlx::Error> {
        let sql = "select g.* from groups g join users_groups ug on g.id = ug.group_id where ug.user_id = $1";
        query_as(sql).bind(user_id).fetch_all(self.db()).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let sql = "select * from users";
        query_as(sql).fetch_all(self.db()).await
    }

    pub async fn find_all_with_groups(&self) -> Result<Vec<UserWithGroups>, sqlx::Error> {
        let users = self.find_all().await?;
        let mut list = Vec::new();
        for user in users {
            let groups = self.groups(user.id).await?;
            list.push(UserWithGroups { user, groups });
        }
        Ok(list)
    }

    pub async fn find(&self, id: Uuid) -> Result<User, sqlx::Error> {
        let sql = "select * from users where id = $1";
        query_as(sql).bind(id).fetch_one(self.db()).await
    }

    pub async fn find_by_username(&self, username: String) -> Result<UserWithGroups, sqlx::Error> {
        let sql = "select * from users where username = $1 or email = $1";
        let user: User = query_as(sql).bind(username).fetch_one(self.db()).await?;
        let groups = self.groups(user.id).await?;
        Ok(UserWithGroups { user, groups })
    }

    pub async fn find_with_groups(&self, id: Uuid) -> Result<UserWithGroups, sqlx::Error> {
        let user = self.find(id).await?;
        let groups = self.groups(id).await?;
        Ok(UserWithGroups { user, groups })
    }

    pub async fn create(&self, dto: UserCreateDto) -> Result<UserWithGroups, sqlx::Error> {
        // create user
        let sql = r#"insert into users 
            (name, phone, role, email, username, password_hash, visible, editable, locked)
        values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        returning *"#;
        let user: User = query_as(sql)
            .bind(dto.name)
            .bind(dto.phone)
            .bind(dto.role)
            .bind(dto.email)
            .bind(dto.username)
            .bind(dto.password_hash)
            .bind(dto.visible)
            .bind(dto.editable)
            .bind(dto.locked)
            .fetch_one(self.db())
            .await?;
        // assign groups
        self.assign(user.id, dto.groups).await?;
        // get groups
        let groups = self.groups(user.id).await?;
        Ok(UserWithGroups { user, groups })
    }

    pub async fn update(
        &self,
        id: Uuid,
        dto: UserUpdateDto,
    ) -> Result<UserWithGroups, sqlx::Error> {
        // update user
        let sql = r#"update users set
            name = $2,
            phone = $3,
            role = $4,
            email = $5,
            username = $6,
            visible = $7,
            editable = $8,
            locked = $9
            updated_at = extract(epoch from now())
        where id = $1 returning *"#;
        let user: User = query_as(sql)
            .bind(id)
            .bind(dto.name)
            .bind(dto.phone)
            .bind(dto.role)
            .bind(dto.email)
            .bind(dto.username)
            .bind(dto.visible)
            .bind(dto.editable)
            .bind(dto.locked)
            .fetch_one(self.db())
            .await?;
        // assign groups
        self.assign(user.id, dto.groups).await?;
        // get groups
        let groups = self.groups(user.id).await?;
        Ok(UserWithGroups { user, groups })
    }

    pub async fn update_password(&self, id: Uuid, dto: PasswordDto) -> Result<(), sqlx::Error> {
        let sql = r#"update users set
            password_hash = $2,
            updated_at = extract(epoch from now())
        where id = $1"#;
        query(sql)
            .bind(id)
            .bind(dto.password_hash)
            .execute(self.db())
            .await?;
        Ok(())
    }
}
