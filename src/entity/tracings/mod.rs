#![allow(unused)]
use crate::libs::prelude::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tracings {
    pub tracing_id: i32,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub data: Data,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Data {
    pub order_id: i32,
    pub status: Status,
    pub subject_id: i32,
    pub subject_name: String,
}

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
pub enum Status {
    Warehouse,
    Driver,
    Complete,
}

impl Tracings {
    pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<Self>
        where E: Executor<'r,Database = Postgres>
    {
        sqlx::query_as(
        "INSERT INTO tracings (order_id,status,subject_id,subject_name) VALUES ($1,$2,$3,$4) RETURNING *")
            .bind(&data.order_id)
            .bind(&data.status.to_string())
            .bind(&data.subject_id)
            .bind(&data.subject_name)
            .fetch_one(db).await
    }

    // pub async fn delete<'r,E>(db: E, order_id: i32) -> sqlx::Result<PgQueryResult> where E: Executor<'r, Database = Postgres> {
    //     sqlx::query("DELETE FROM tracings WHERE order_id = $1")
    //         .bind(order_id)
    //         .execute(db).await
    // }

    pub async fn archive<'r,E>(db: E, order_id: i32) -> sqlx::Result<PgQueryResult> where E: Executor<'r, Database = Postgres> {
        sqlx::query("WITH _ as (
            INSERT INTO tracings_archive (tracing_id,order_id,status,subject_id,subject_name,created_at)
            SELECT tracing_id,order_id,status,subject_id,subject_name,created_at FROM tracings WHERE order_id = $1
        )
        DELETE FROM tracings WHERE order_id = $1")
            .bind(order_id)
            .execute(db).await
    }

    pub async fn find_by_order_id<'r,E>(db: E, order_id: i32) -> sqlx::Result<Option<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM tracings WHERE order_id = $1")
            .bind(order_id)
            .fetch_optional(db).await
    }

    pub async fn list_by_order_id<'r,E>(db: E, order_id: i32) -> sqlx::Result<Vec<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM tracings WHERE order_id = $1")
            .bind(order_id)
            .fetch_all(db).await
    }

    pub async fn list_by_subject_id<'r,E>(db: E, sub_id: i32) -> sqlx::Result<Option<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM tracings WHERE subject_id = $1 LIMIT 1")
            .bind(sub_id)
            .fetch_optional(db).await
    }
}



impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Status {
    fn decode(value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(Status::from_str(value.as_str()?)?)
    }
}

impl sqlx::Type<sqlx::Postgres> for Status {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

