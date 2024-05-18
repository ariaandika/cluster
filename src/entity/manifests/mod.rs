#![allow(unused)]
use crate::libs::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Manifests {
    pub manifest_id: i32,
    pub sales_id: i32,
    pub driver_id: i32,
    pub wh_from_id: i32,
    pub wh_to_id: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub sales_id: i32,
    pub driver_id: i32,
    pub wh_from_id: i32,
    pub wh_to_id: i32,
}

impl Manifests {
    pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<Self>
        where E: Executor<'r,Database = Postgres>
    {
        sqlx::query_as(
        "INSERT INTO manifests (sales_id,driver_id,wh_from_id,wh_to_id) VALUES ($1,$2,$3,$4) RETURNING *")
            .bind(&data.sales_id)
            .bind(&data.driver_id)
            .bind(&data.wh_from_id)
            .bind(&data.wh_to_id)
            .fetch_one(db).await
    }

    pub async fn complete<'r,E>(db: E, manifest_id: i32) -> sqlx::Result<Self>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("UPDATE manifests SET completed_at = now() WHERE manifest_id = $1 RETURNING *")
            .bind(manifest_id)
            .fetch_one(db).await
    }
}

#[derive(Debug, FromRow)]
pub struct ManifestOrders {
    pub manifest_id: i32,
    pub order_id: i32,
}

impl ManifestOrders {
    pub async fn create<'r,E>(db: E, order_id: i32, manifest_id: i32) -> sqlx::Result<PgQueryResult>
        where E: Executor<'r,Database = Postgres>
    {
        // TODO: bulk insert
        sqlx::query("INSERT INTO manifest_orders (order_id,manifest_id) VALUES ($1,$2)")
            .bind(order_id)
            .bind(manifest_id)
            .execute(db).await
    }

    pub async fn list_by_manifest_id<'r,E>(db: E, manifest_id: i32) -> sqlx::Result<Vec<Self>>
        where E: Executor<'r,Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM manifest_orders WHERE manifest_id = $1")
            .bind(manifest_id)
            .fetch_all(db).await
    }
}

