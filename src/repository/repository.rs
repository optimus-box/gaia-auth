use sqlx::{postgres::PgRow, query, query_as, FromRow, Pool};
use uuid::Uuid;

pub trait Repository<T: for<'r> FromRow<'r, PgRow> + Send + Unpin> {
    fn new(db: Pool<sqlx::Postgres>) -> Self;
    fn db(&self) -> &sqlx::Pool<sqlx::Postgres>;
    fn table(&self) -> &str;

    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error> {
        let sql = format!("select * from {}", self.table());
        query_as(&sql).fetch_all(self.db()).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<T, sqlx::Error> {
        let sql = format!("select * from {} where id = $1", self.table());
        query_as(&sql).bind(id).fetch_one(self.db()).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        let sql = format!("delete from {} where id = $1", self.table());
        query(&sql).bind(id).execute(self.db()).await?;
        Ok(())
    }
}
