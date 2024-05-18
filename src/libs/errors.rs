use strum_macros::{EnumMessage, EnumProperty, Display};
use axum::{extract::Json, http::StatusCode, response::Response};
use serde_json::Value;

#[derive(Debug, Display, EnumMessage, EnumProperty)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    /// NOTE: Recoverable Error

    /// 422 Login specific error
    InvalidCredential,
    /// 422 Recoverable error by user
    // UnprocessableEntity

    /// NOTE: Unrecoverable Error

    /// 401 General auth error
    Unauthenticated,
    InvalidToken(String),
    TokenExpired,
    InvalidMetadata, /// Maybe token metadata schema is changed
    Unauthorized,   /// 403
    NotFound,       /// 404
    InternalError,  /// 500

    /// Not Implemented
    #[strum(message = "Input invalid", props(status = "41"))]
    Validation(Value),
    Duplicate(String),
    BadRequest(String),
    Custom { error: String, message: String, value: Value }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use Error::*;

        let error = self.to_string();
        let (status,message) = match self {
            InvalidCredential => (StatusCode::UNPROCESSABLE_ENTITY,"Invalid Credential".into()),

            Unauthenticated => (StatusCode::UNAUTHORIZED, "Authentication is required".into()),
            InvalidToken(message) => (StatusCode::UNAUTHORIZED, message),
            TokenExpired | InvalidMetadata => (StatusCode::UNAUTHORIZED, "Token Expired".into()),
            Unauthorized => (StatusCode::FORBIDDEN, "You are not allowed to access this resource".into()),

            NotFound => (StatusCode::NOT_FOUND, "Not Found".into()),
            InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into()),
            BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),

            Validation(_) | Duplicate(_) | Custom { .. }
                => (StatusCode::NOT_IMPLEMENTED, "Not yet implemented".into()),
        };

        (status, Json(ErrorResponse { message, error, value: Value::default() })).into_response()
    }
}

impl std::error::Error for Error { }

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    message: String,
    #[serde(flatten, skip_serializing_if = "Value::is_null")]
    value: Value,
}

pub type Result<T = Response> = std::result::Result<T, Error>;

impl Error {
    pub fn fatal<T>(value: T) -> Self where T: std::fmt::Display {
        tracing::error!("{value}");
        Self::InternalError
    }
    pub fn debug<T>(_value: T) -> Self where T: std::error::Error {
        dbg!("{_value:#?}");
        Self::InternalError
    }
}

