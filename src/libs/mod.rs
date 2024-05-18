pub mod errors;
pub mod database;
pub mod extractors;
pub mod password;
pub mod jwt;

pub mod prelude {
    pub use serde::{Serialize, Deserialize, de::DeserializeOwned};
    pub use sqlx::{prelude::*, postgres::*, types::chrono::{DateTime, Utc}};
    pub use serde_json::{Value, Map, json};
    pub use strum_macros::{Display as EnumDisplay, EnumString};

    pub use std::sync::Arc;
    pub use std::str::FromStr;
    pub use super::errors::{Result, Error};
}

pub mod prelude_router {
    pub use axum::async_trait;
    pub use axum::routing::{Router, get, post};
    pub use axum::extract::{Extension, Path, Query, FromRequest, FromRequestParts};
    pub use axum::response::{IntoResponse, Response};
    pub use axum::http::{StatusCode, Request, self};
    pub use super::extractors::{Json, JsonData};
}

pub mod prelude_entity {
    pub use crate::entity::auth::{UserToken, Auth, Sales, Customer, self};
}

