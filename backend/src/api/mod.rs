use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::WIState;

mod player;
pub(crate) mod runs;

pub fn router() -> Router<Arc<WIState>> {
    Router::new()
        .route("/runs", get(runs::list))
        .route("/runs/{id}", get(runs::get))
        .route("/player/{username}", get(player::get))
        .route("/player", post(player::update))
}
