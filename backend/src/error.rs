use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DbErr;
use serde_json::json;

pub type WIError = (StatusCode, Json<serde_json::Value>);

pub fn make_error(code: StatusCode, message: String) -> WIError {
    (code, Json(json!({"error": message})))
}

pub fn db_error(e: DbErr) -> WIError {
    make_error(StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}"))
}
// pub enum WIError {
//     Parse(String),
//     DB(String),
//     Env(String),
//     // NotFound(String),
// }

// impl IntoResponse for WIError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, msg) = match self {
//             Self::Parse(m) => (StatusCode::BAD_REQUEST, m),
//             Self::DB(m) | Self::Env(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
//             // Self::NotFound(m) => (StatusCode::NOT_FOUND, m),
//         };
//         (status, Json(json!({ "error": msg }))).into_response()
//     }
// }
