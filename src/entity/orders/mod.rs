#![allow(unused)]
use crate::libs::prelude::*;
use crate::entity::packages::Packages;
use crate::entity::users;
use crate::entity::tracings;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Orders {
    pub order_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub destination: Address
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub detail: String,
    pub kelurahan: String,
    pub kecamatan: String,
    pub kabupaten: String,
    pub provinsi: String,
    pub kodepos: i32,
}

// impl<'r> sqlx::Encode<'r, sqlx::Postgres> for Address {
//     fn decode(value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
//         let json = value.as_str()?;
//         Ok(serde_json::from_str::<Address>(json)?)
//     }
// }

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Address {
    fn decode(value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let json = value.as_str()?;
        Ok(serde_json::from_str::<Address>(json)?)
    }
}

impl sqlx::Type<sqlx::Postgres> for Address {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("JSON")
    }
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub destination: Address,
}

impl Orders {
    pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<Self>
        where E: Executor<'r,Database = Postgres>
    {
        sqlx::query_as(
        "INSERT INTO orders (sender_id,receiver_id,destination) VALUES ($1,$2,$3) RETURNING *")
            .bind(&data.sender_id)
            .bind(&data.receiver_id)
            .bind(serde_json::to_value(&data.destination).unwrap())
            .fetch_one(db).await
    }

    pub async fn list<'r,E>(db: E, limit: i32, page: i32) -> sqlx::Result<Vec<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM orders LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(page)
            .fetch_all(db).await
    }

    pub async fn find_by_id<'r,E>(db: E, order_id: i32) -> sqlx::Result<Option<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM orders WHERE order_id = $1")
            .bind(order_id)
            .fetch_optional(db).await
    }

    pub async fn list_packages<'r,E>(db: E, order_id: i32) -> sqlx::Result<Vec<Packages>>
        where E: Executor<'r, Database = Postgres>
    {
        Packages::list_by_order_id(db, order_id).await
    }

}




