use crate::libs::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Warehouses {
    pub wh_id: i32,
    pub name: String,
    pub wh_type: WhType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub name: String,
    pub wh_type: WhType,
}

#[derive(Debug, EnumDisplay, Serialize, Deserialize, EnumString, PartialEq)]
pub enum WhType {
    Counter,
    Warehouse,
    DistCenter,
}

pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<Warehouses>
    where E: Executor<'r,Database = Postgres>
{
    sqlx::query_as(
    "INSERT INTO warehouses (name,wh_type) VALUES ($1,$2) RETURNING *")
        .bind(&data.name)
        .bind(&data.wh_type.to_string())
        .fetch_one(db).await
}

pub async fn list<'r,E>(db: E, limit: i32, page: i32) -> sqlx::Result<Vec<Warehouses>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as("SELECT * FROM warehouses LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(page)
        .fetch_all(db).await
}

pub async fn find_by_id<'r,E>(db: E, wh_id: i32) -> sqlx::Result<Option<Warehouses>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as("SELECT * FROM warehouses WHERE wh_id = $1")
        .bind(wh_id)
        .fetch_optional(db).await
}




impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WhType {
    fn decode(value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(WhType::from_str(value.as_str()?)?)
    }
}

impl sqlx::Type<sqlx::Postgres> for WhType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}



#[sqlx::test]
async fn test(db: PgPool) -> sqlx::Result<()> {
    let wh = Data {
        name: "Clock".into(),
        wh_type: WhType::Counter,
    };

    let wh = create(&db,&wh).await?;

    let whs = list(&db, 100, 0).await?;
    let found = whs.into_iter().find(|u|*u == wh).is_some();

    assert!(found);

    assert!(find_by_id(&db,wh.wh_id).await?.is_some());

    Ok(())
}
