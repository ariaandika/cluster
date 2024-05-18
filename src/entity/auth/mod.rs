use axum::http::request::Parts;
use crate::libs::prelude::*;
use crate::libs::prelude_router::*;
use crate::entity::employees;
use crate::entity::users::{Users, Role};

pub struct Auth<T>(pub T);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub exp: usize,
    #[serde(flatten)]
    pub user: Users,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub role_data: Value,
}

pub struct Sales(pub UserToken);
pub struct Customer(pub UserToken);
pub struct Admin(pub UserToken);
pub struct Driver(pub UserToken);
pub struct Courier(pub UserToken);

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesData {
    pub wh_name: String,
    pub wh_id: i32,
}

impl Sales {
    pub fn data(&self) -> Result<SalesData> {
        // NOTE: clone
        serde_json::from_value(self.0.role_data.clone()).ok().ok_or(Error::InvalidMetadata)
    }

    pub async fn create_data<'r,E>(db: E, user: &Users) -> Result<Value> where E: Executor<'r, Database = Postgres> {
        let res = employees::find_wh_by_uid(db, user.user_id).await?.ok_or(Error::Unauthorized)?;
        Ok(serde_json::to_value(SalesData { wh_name: res.name, wh_id: res.wh_id }).map_err(Error::fatal)?)
    }
}

impl UserToken {
    fn new(user: Users, role_data: Value) -> Self {
        Self {
            exp: crate::libs::jwt::one_week_expiration(),
            user, role_data
        }
    }

    pub async fn sign(user: Users, role_data: Value) -> Result<String> {
        crate::libs::jwt::sign(Self::new(user, role_data))
    }

    pub fn verify(token: String) -> Result<Self> {
        crate::libs::jwt::verify::<Self>(token)
    }
}

pub async fn create_role_token_data<'r,E>(db: E, user: &Users) -> Result<Value>
    where E: Executor<'r, Database = Postgres>
{
    match &user.data.role {
        Role::Admin => Admin::create_token_data(db, &user).await,
        Role::Driver => Driver::create_token_data(db, &user).await,
        Role::Courier => Courier::create_token_data(db, &user).await,
        Role::Customer => Customer::create_token_data(db, &user).await,
        Role::Sales => Sales::create_token_data(db, &user).await,
    }
}

macro_rules! default_role_trait {
    ($this:ty, $role:expr) => {
        impl RoleTrait for $this {
            fn new(value: UserToken) -> Self {
                Self(value)
            }
            fn assert_role(role: &Role) -> bool {
                role == &$role
            }
        }
    };
    ($this:ty, $role:expr, $fnn:expr) => {
        impl RoleTrait for $this {
            async fn create_token_data<'r,E>(db: E, user: &Users) -> Result<Value> where E: Executor<'r,Database = Postgres> {
                $fnn(db, user).await
            }
            fn new(value: UserToken) -> Self {
                Self(value)
            }
            fn assert_role(role: &Role) -> bool {
                role == &$role
            }
        }
    }
}

pub trait RoleTrait {
    fn new(value: UserToken) -> Self where Self: Sized;
    fn assert_role(_role: &Role) -> bool;

    #[allow(async_fn_in_trait)]
    async fn create_token_data<'r,E>(_db: E, _user: &Users) -> Result<Value> where E: Executor<'r,Database = Postgres> { Ok(Value::Null) }
}

default_role_trait!(Sales, Role::Sales, Sales::create_data);
default_role_trait!(Customer, Role::Customer);
default_role_trait!(Admin, Role::Admin);
default_role_trait!(Driver, Role::Driver);
default_role_trait!(Courier, Role::Courier);

#[async_trait]
impl<S> FromRequestParts<S> for UserToken where S: Send + Sync {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        let token = if let Some(c) = take_cookie(parts) { c }
        else if let Some(c) = take_bearer(parts) { c }
        else { return Err(Error::Unauthenticated) };
        Self::verify(token)
    }
}

#[async_trait]
impl<T,S> FromRequestParts<S> for Auth<T> where S: Send + Sync, T: RoleTrait {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        let user_token = UserToken::from_request_parts(parts,_state).await?;
        if !T::assert_role(&user_token.user.data.role) {
            tracing::info!("UNAUTHORIZED {:?}",&user_token.user);
            return Err(Error::Unauthorized);
        }
        Ok(Self(T::new(user_token)))
    }
}

fn take_cookie(parts: &Parts) -> Option<String> {
    let res = parts.headers.get("cookie")?
        .to_str().ok()?
        .split("; ")
        .find(|e|e.starts_with("access_token="))?;

    Some(res.replacen("access_token=","",1))
}

fn take_bearer(parts: &Parts) -> Option<String> {
    let res = parts.headers.get("authorization")?
        .to_str().ok()?
        .split_once(" ")?;

    Some(res.1.to_string())
}

