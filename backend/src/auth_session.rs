use std::sync::Arc;

use axum::extract::FromRequestParts;
use better_auth::CurrentSession;
use reqwest::StatusCode;

use crate::{
    auth_entities::AppAdapter,
    error::{make_error, WIError},
    WIState,
};

pub struct WISession(pub CurrentSession<AppAdapter>);

impl FromRequestParts<Arc<WIState>> for WISession {
    type Rejection = WIError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<WIState>,
    ) -> Result<Self, Self::Rejection> {
        let session = CurrentSession::<AppAdapter>::from_request_parts(parts, &state.auth)
            .await
            .map_err(|_| make_error(StatusCode::UNAUTHORIZED, "Unauthorized".into()))?;
        Ok(WISession(session))
    }
}
