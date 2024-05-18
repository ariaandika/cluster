#![allow(unused)]
use crate::libs::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Packages {
    pub package_id: i32,
    pub order_id: i32,
    pub name: String,
    pub weight: f32,
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub order_id: i32,
    pub name: String,
    pub weight: f32,
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

impl Packages {
    pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<PgQueryResult>
        where E: Executor<'r,Database = Postgres>
    {
        sqlx::query(
        "INSERT INTO packages (order_id,name,weight,length,width,height) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(&data.order_id)
            .bind(&data.name)
            .bind(&data.weight)
            .bind(&data.length)
            .bind(&data.width)
            .bind(&data.height)
            .execute(db).await
    }

    pub async fn list_by_order_id<'r,E>(db: E, order_id: i32) -> sqlx::Result<Vec<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM packages WHERE order_id = $1")
            .bind(order_id)
            .fetch_all(db).await
    }
}


