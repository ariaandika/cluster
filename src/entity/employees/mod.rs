use crate::libs::prelude::*;
use crate::entity::warehouses::Warehouses;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employees {
    pub user_id: i32,
    pub wh_id: i32,
}

pub async fn create<'r,E>(db: E, data: &Employees) -> sqlx::Result<PgQueryResult>
    where E: Executor<'r,Database = Postgres>
{
    sqlx::query("INSERT INTO employees (user_id,wh_id) VALUES ($1,$2)")
        .bind(&data.user_id)
        .bind(&data.wh_id)
        .execute(db).await
}

pub async fn find_wh_by_uid<'r,E>(db: E, user_id: i32) -> sqlx::Result<Option<Warehouses>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as(
    "SELECT * FROM employees e LEFT JOIN warehouses w ON w.wh_id = e.wh_id WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(db).await
}


