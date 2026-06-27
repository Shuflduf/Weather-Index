use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum WIError {
    Parse(String),
    DB(String),
    // NotFound(String),
}

impl IntoResponse for WIError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::Parse(m) => (StatusCode::BAD_REQUEST, m),
            Self::DB(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            // Self::NotFound(m) => (StatusCode::NOT_FOUND, m),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
