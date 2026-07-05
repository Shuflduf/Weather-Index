use std::sync::Arc;

use axum::{routing::get, Router};

use crate::WIState;

mod runs;

pub fn router() -> Router<Arc<WIState>> {
    Router::new()
        .route("/runs", get(runs::list))
        .route("/runs/{id}", get(runs::get))
}
