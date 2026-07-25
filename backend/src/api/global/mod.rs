use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};
use serde::Serialize;

use crate::{
    entity::run_report,
    error::{db_error, make_error, WIError},
    WIState,
};

#[derive(FromQueryResult)]
pub struct GlobalStats {
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
pub struct GlobalStatsDTO {
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

impl From<GlobalStats> for GlobalStatsDTO {
    fn from(value: GlobalStats) -> Self {
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

pub async fn aggregate_stats(
    db: &DatabaseConnection,
    user_id: Option<&String>,
) -> Result<GlobalStats, WIError> {
    let sum_columns = [
        (run_report::Column::TimeAliveSeconds, "time_alive_seconds"),
        (run_report::Column::StagesCompleted, "stages_completed"),
        (run_report::Column::ItemsCollected, "items_collected"),
        (run_report::Column::DronesPurchased, "drones_purchased"),
        (run_report::Column::EliteKills, "elite_kills"),
        (run_report::Column::MinionKills, "minion_kills"),
        (run_report::Column::DamageDealt, "damage_dealt"),
        (run_report::Column::MinionDamageDealt, "minion_damage_dealt"),
        (run_report::Column::DamageTaken, "damage_taken"),
        (run_report::Column::HealingRecieved, "healing_recieved"),
        (run_report::Column::GoldCollected, "gold_collected"),
        (run_report::Column::BloodPurchases, "blood_purchases"),
        (run_report::Column::LunarPurchases, "lunar_purchases"),
        (run_report::Column::DistanceTraveled, "distance_traveled"),
    ];
    let highest_columns = [
        (
            run_report::Column::HighestDamageDealt,
            "highest_damage_dealt",
        ),
        (run_report::Column::HighestLevel, "highest_level"),
    ];
    let mut row = run_report::Entity::find().select_only();
    if let Some(id) = user_id {
        row = row.filter(run_report::Column::UserId.eq(id));
    }

    sum_columns.iter().for_each(|c| {
        row = row.clone().column_as(c.0.sum().cast_as("BIGINT"), c.1);
    });
    highest_columns.iter().for_each(|c| {
        row = row.clone().column_as(c.0.max().cast_as("BIGINT"), c.1);
    });

    let row = row
        .into_model::<GlobalStats>()
        .one(db)
        .await
        .map_err(db_error)?
        .ok_or(make_error(StatusCode::NOT_FOUND, "oops".into()))?;
    Ok(row)
}

pub async fn get(State(state): State<Arc<WIState>>) -> Result<Json<GlobalStatsDTO>, WIError> {
    Ok(Json(aggregate_stats(&state.db, None).await?.into()))
}
