use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde::Serialize;

use crate::{
    entity::{run_report, user},
    error::{db_error, make_error, WIError},
    WIState,
};

#[derive(FromQueryResult)]
pub struct LifetimeStats {
    pub time_alive_seconds: Option<i64>,
    pub stages_completed: Option<i64>,
    pub score: Option<i64>,
    pub items_collected: Option<i64>,
    pub drones_purchased: Option<i64>,
    pub turrets_purchased: Option<i64>,
    pub kills: Option<i64>,
    pub elite_kills: Option<i64>,
    pub minion_kills: Option<i64>,
    pub deaths: Option<i64>,
    pub damage_dealt: Option<i64>,
    pub minion_damage_dealt: Option<i64>,
    pub damage_taken: Option<i64>,
    pub highest_damage_dealt: Option<i64>,
    pub healing_recieved: Option<i64>,
    pub highest_level: Option<i64>,
    pub gold_collected: Option<i64>,
    pub purchases: Option<i64>,
    pub gold_purchases: Option<i64>,
    pub blood_purchases: Option<i64>,
    pub lunar_purchases: Option<i64>,
    pub distance_traveled: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LifetimeStatsDTO {
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

impl From<LifetimeStats> for LifetimeStatsDTO {
    fn from(value: LifetimeStats) -> Self {
        Self {
            time_alive_seconds: value.time_alive_seconds.unwrap_or(0),
            stages_completed: value.stages_completed.unwrap_or(0),
            score: value.score.unwrap_or(0),
            items_collected: value.items_collected.unwrap_or(0),
            drones_purchased: value.drones_purchased.unwrap_or(0),
            turrets_purchased: value.turrets_purchased.unwrap_or(0),
            kills: value.kills.unwrap_or(0),
            elite_kills: value.elite_kills.unwrap_or(0),
            minion_kills: value.minion_kills.unwrap_or(0),
            deaths: value.deaths.unwrap_or(0),
            damage_dealt: value.damage_dealt.unwrap_or(0),
            minion_damage_dealt: value.minion_damage_dealt.unwrap_or(0),
            damage_taken: value.damage_taken.unwrap_or(0),
            highest_damage_dealt: value.highest_damage_dealt.unwrap_or(0),
            healing_recieved: value.healing_recieved.unwrap_or(0),
            highest_level: value.highest_level.unwrap_or(0),
            gold_collected: value.gold_collected.unwrap_or(0),
            purchases: value.purchases.unwrap_or(0),
            gold_purchases: value.gold_purchases.unwrap_or(0),
            blood_purchases: value.blood_purchases.unwrap_or(0),
            lunar_purchases: value.lunar_purchases.unwrap_or(0),
            distance_traveled: value.distance_traveled.unwrap_or(0),
        }
    }
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<LifetimeStatsDTO>, WIError> {
    let player = user::Entity::find()
        .filter(user::Column::Username.eq(&username))
        .one(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query users: {e}"),
            )
        })?
        .ok_or_else(|| {
            make_error(
                StatusCode::NOT_FOUND,
                format!("User `{username}` not found"),
            )
        })?;
    let row = run_report::Entity::find()
        .select_only()
        .filter(run_report::Column::UserId.eq(&player.id))
        .column_as(
            run_report::Column::TimeAliveSeconds.sum().cast_as("BIGINT"),
            "time_alive_seconds",
        )
        .column_as(
            run_report::Column::StagesCompleted.sum().cast_as("BIGINT"),
            "stages_completed",
        )
        .column_as(run_report::Column::Score.sum().cast_as("BIGINT"), "score")
        .column_as(
            run_report::Column::ItemsCollected.sum().cast_as("BIGINT"),
            "items_collected",
        )
        .column_as(
            run_report::Column::DronesPurchased.sum().cast_as("BIGINT"),
            "drones_purchased",
        )
        .column_as(
            run_report::Column::TurretsPurchased.sum().cast_as("BIGINT"),
            "turrets_purchased",
        )
        .column_as(run_report::Column::Kills.sum().cast_as("BIGINT"), "kills")
        .column_as(
            run_report::Column::EliteKills.sum().cast_as("BIGINT"),
            "elite_kills",
        )
        .column_as(
            run_report::Column::MinionKills.sum().cast_as("BIGINT"),
            "minion_kills",
        )
        .column_as(run_report::Column::Deaths.sum().cast_as("BIGINT"), "deaths")
        .column_as(
            run_report::Column::DamageDealt.sum().cast_as("BIGINT"),
            "damage_dealt",
        )
        .column_as(
            run_report::Column::MinionDamageDealt
                .sum()
                .cast_as("BIGINT"),
            "minion_damage_dealt",
        )
        .column_as(
            run_report::Column::DamageTaken.sum().cast_as("BIGINT"),
            "damage_taken",
        )
        .column_as(
            run_report::Column::HighestDamageDealt
                .max()
                .cast_as("BIGINT"),
            "highest_damage_dealt",
        )
        .column_as(
            run_report::Column::HealingRecieved.sum().cast_as("BIGINT"),
            "healing_recieved",
        )
        .column_as(
            run_report::Column::HighestLevel.max().cast_as("BIGINT"),
            "highest_level",
        )
        .column_as(
            run_report::Column::GoldCollected.sum().cast_as("BIGINT"),
            "gold_collected",
        )
        .column_as(
            run_report::Column::Purchases.sum().cast_as("BIGINT"),
            "purchases",
        )
        .column_as(
            run_report::Column::GoldPurchases.sum().cast_as("BIGINT"),
            "gold_purchases",
        )
        .column_as(
            run_report::Column::BloodPurchases.sum().cast_as("BIGINT"),
            "blood_purchases",
        )
        .column_as(
            run_report::Column::LunarPurchases.sum().cast_as("BIGINT"),
            "lunar_purchases",
        )
        .column_as(
            run_report::Column::DistanceTraveled.sum().cast_as("BIGINT"),
            "distance_traveled",
        )
        .into_model::<LifetimeStats>()
        .one(&state.db)
        .await
        .map_err(db_error)?
        .ok_or(make_error(StatusCode::NOT_FOUND, "oops".into()))?;
    return Ok(Json(row.into()));
}
