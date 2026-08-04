use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::WIState;

mod data;
mod player;
pub(crate) mod runs;
mod stats;

fn public_router() -> Router<Arc<WIState>> {
    Router::new()
        .route("/runs", get(runs::list::get))
        .route("/runs/{id}", get(runs::get))
        .route("/player/{username}", get(player::get))
        .route("/stats/sum", get(stats::combined::sum))
        .route("/stats/avg", get(stats::combined::avg))
        .route("/stats/survivors", get(stats::survivors::get))
        .route("/stats/stages", get(stats::stages::get))
        .route("/stats/overall", get(stats::overall::get))
        .route("/stats/difficulties", get(stats::difficulties::get))
        .route("/stats/artifacts", get(stats::artifacts::get))
        .route("/data/items", get(data::items))
        .route("/data/tiers", get(data::tiers))
        .route("/data/bodies", get(data::bodies))
        .route("/data/endings", get(data::endings))
        .route("/data/scoring", get(data::scoring))
        .route("/data/artifacts", get(data::artifacts))
        .route("/data/difficulties", get(data::difficulties))
        .route("/data/environments", get(data::environments))
        .route("/data/equipment", get(data::equipment))
        .route("/data/skills", get(data::skills))
}

fn private_router() -> Router<Arc<WIState>> {
    Router::new()
        .route("/player", post(player::update))
        .route("/runs/new", post(runs::new::post))
}

pub fn router() -> Router<Arc<WIState>> {
    public_router().merge(private_router())
}
