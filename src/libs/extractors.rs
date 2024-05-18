use axum::{async_trait, body::Bytes, extract::{FromRequest, Request}, response::IntoResponse};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use crate::libs::errors::{Result, Error};

pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self> {
        let bytes = Bytes::from_request(req, state).await.map_err(bad_request)?;
        let ok = serde_json::from_slice::<T>(&bytes).map_err(bad_request)?;
        Ok(Self(ok))
    }
}

pub fn bad_request<T>(value: T) -> Error where T: std::fmt::Display {
    Error::BadRequest(value.to_string())
}

impl<T> IntoResponse for Json<T> where T: serde::Serialize {
    fn into_response(self) -> axum::response::Response {
        axum::extract::Json(self.0).into_response()
    }
}

impl<T> Json<T> where T: Serialize {
    pub fn data(value: T) -> Result {
        Ok(Json(json!{{ "data": value }}).into_response())
    }
}


pub struct JsonData<T>(pub T);

impl<T> IntoResponse for JsonData<T> where T: serde::Serialize {
    fn into_response(self) -> axum::response::Response {
        Json::data(self.0).into_response()
    }
}

