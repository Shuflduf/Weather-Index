use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts};
use reqwest::StatusCode;

use crate::{
    error::{make_error, WIError},
    WIState,
};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub _session_id: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<WIState>: FromRef<S>,
{
    type Rejection = WIError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| make_error(StatusCode::UNAUTHORIZED, "Missing token".into()))?;

        let state = Arc::<WIState>::from_ref(state);
        let session = state
            .auth
            .session_manager()
            .get_session(token)
            .await
            .map_err(|e| {
                make_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Auth error: {e}"),
                )
            })?
            .ok_or_else(|| {
                make_error(StatusCode::UNAUTHORIZED, "Invalid or expired token".into())
            })?;

        Ok(Self {
            user_id: session.user_id,
            _session_id: session.id,
        })
    }
}
