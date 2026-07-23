use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    api::player::PlayerGetResponse,
    error::{make_error, WIError},
    WIState,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifetimeStats {
    pub time_alive_seconds: i64,
    pub stages_completed: i64,
    pub score: i64,
    pub items_collected: i64,
    pub drones_purchased: i64,
    pub turrets_purchased: i64,
    pub kills: i64,
    pub elite_kills: i64,
    pub minion_kills: i64,
    pub deaths: i64,
    pub damage_dealt: i64,
    pub minion_damage_dealt: i64,
    pub damage_taken: i64,
    // NOT A SUM
    pub highest_damage_dealt: i64,
    pub healing_recieved: i64,
    // NOT A SUM
    pub highest_level: i64,
    pub gold_collected: i64,
    pub purchases: i64,
    pub gold_purchases: i64,
    pub blood_purchases: i64,
    pub lunar_purchases: i64,
    pub distance_traveled: i64,
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<LifetimeStats>, WIError> {
    return Err(make_error(StatusCode::NOT_IMPLEMENTED, "dlksfsdf".into()));
}
