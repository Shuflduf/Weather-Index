use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::WIState;

mod global;
mod player;
pub(crate) mod runs;

pub fn router() -> Router<Arc<WIState>> {
    Router::new()
        .route("/runs", get(runs::list::get))
        .route("/runs/{id}", get(runs::get))
        .route("/runs/new", get(runs::new::post))
        .route("/player/{username}", get(player::get))
        .route("/player/lifetime/{username}", get(player::lifetime::get))
        .route("/player", post(player::update))
        .route("/global/sum", get(global::sum))
        .route("/global/avg", get(global::avg))
}
