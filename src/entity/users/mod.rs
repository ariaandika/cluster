use crate::libs::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Users {
    pub user_id: i32,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub data: Data,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Snapshot {
    pub snapshot_id: i32,
    pub user_id: Option<i32>,
    pub name: String,
    pub phone: String,
    pub role: String,
    pub metadata: Value,
    pub snapshoted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Data {
    pub name: String,
    pub phone: String,
    #[serde(default,skip_serializing)]
    pub password: String,
    pub role: Role,
    pub metadata: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumDisplay, EnumString, PartialEq)]
pub enum Role {
    Admin,
    Driver,
    Courier,
    #[default]
    Customer,
    Sales,
}

pub async fn create<'r,E>(db: E, data: &Data) -> sqlx::Result<Users>
    where E: Executor<'r,Database = Postgres>
{
    sqlx::query_as(
    "INSERT INTO users (name,phone,password,role,metadata) VALUES ($1,$2,$3,$4,$5) RETURNING *")
        .bind(&data.name)
        .bind(&data.phone)
        .bind(&data.password)
        .bind(&data.role.to_string())
        .bind(&data.metadata)
        .fetch_one(db).await
}

pub async fn list<'r,E>(db: E, limit: i32, page: i32) -> sqlx::Result<Vec<Users>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as("SELECT * FROM users LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(page)
        .fetch_all(db).await
}

pub async fn find_by_id<'r,E>(db: E, user_id: i32) -> sqlx::Result<Option<Users>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(db).await
}

pub async fn find_by_phone<'r,E>(db: E, phone: &String) -> sqlx::Result<Option<Users>>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as("SELECT * FROM users WHERE phone = $1")
        .bind(phone)
        .fetch_optional(db).await
}

/// Error when associated user_id not found
pub async fn snapshot_user<'r,E>(db: E, user_id: &i32) -> sqlx::Result<(i32,String)>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as(
    "INSERT INTO users_snapshot (user_id,name,phone,role,metadata)
    SELECT user_id,name,phone,role,metadata FROM users WHERE user_id = $1 RETURNING snapshot_id,name")
        .bind(user_id)
        .fetch_one(db).await
}

pub async fn snapshot_anon<'r,E>(db: E, name: &String, phone: &String, role: &Role) -> sqlx::Result<(i32,String)>
    where E: Executor<'r, Database = Postgres>
{
    sqlx::query_as(
    "INSERT INTO users_snapshot (name,phone,role,metadata) VALUES ($1,$2,$3,'{}'::json) RETURNING snapshot_id,name")
        .bind(name)
        .bind(phone)
        .bind(role.to_string())
        .fetch_one(db).await
}


impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Role {
    fn decode(value: <sqlx::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(Role::from_str(value.as_str()?)?)
    }
}

impl sqlx::Type<sqlx::Postgres> for Role {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl Snapshot {
    pub async fn find_by_id<'r,E>(db: E, snapshot_id: i32) -> sqlx::Result<Option<Self>>
        where E: Executor<'r, Database = Postgres>
    {
        sqlx::query_as("SELECT * FROM users_snapshot WHERE snapshot_id = $1")
            .bind(snapshot_id)
            .fetch_optional(db).await
    }
}

#[sqlx::test]
async fn test(db: PgPool) -> sqlx::Result<()> {
    let user = Data {
        name: "Sam".into(),
        phone: "089636632555".into(),
        password: crate::libs::password::hash(b"passwd123").unwrap(),
        role: Role::Customer,
        ..Default::default()
    };

    let user = create(&db,&user).await?;

    let users = list(&db, 100, 0).await?;
    let found = users.into_iter().find(|u|*u == user).is_some();

    assert!(found);

    assert!(find_by_id(&db,user.user_id).await?.is_some());
    assert!(find_by_phone(&db,&user.data.phone).await?.is_some());

    let _snapshot = snapshot_user(&db, &user.user_id).await?;
    let _snapshot = snapshot_anon(&db, &"Jack".into(), &"089636632557".into(), &Role::Customer).await?;

    Ok(())
}
