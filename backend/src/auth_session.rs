use std::sync::Arc;

use axum::extract::FromRequestParts;
use better_auth::CurrentSession;
use reqwest::StatusCode;

use crate::{
    auth_entities::AppHookedAdapter,
    error::{make_error, WIError},
    WIState,
};

pub struct WISession(pub CurrentSession<AppHookedAdapter>);

impl FromRequestParts<Arc<WIState>> for WISession {
    type Rejection = WIError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<WIState>,
    ) -> Result<Self, Self::Rejection> {
        let session = CurrentSession::<AppHookedAdapter>::from_request_parts(parts, &state.auth)
            .await
            .map_err(|_| make_error(StatusCode::UNAUTHORIZED, "Unauthorized".into()))?;
        Ok(WISession(session))
    }
}
