use std::error::Error;

use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::{
    entity::run_report::{self},
    scoring_table::ScoringTable,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunReportDTO {
    pub survivor: String,
    pub ending: String,
    pub start_time: String,
    pub difficulty: String,
    pub time_alive_seconds: i64,
    pub artifacts: Vec<String>,
    pub stages_completed: i16,

    // items
    pub items: serde_json::Value,
    pub items_collected: i32,

    //
    // drones
    pub drones_purchased: i16,
    pub turrets_purchased: i16,

    // combat
    pub kills: i32,
    pub elite_kills: i32,
    pub minion_kills: i32,
    pub deaths: i32,

    // damage
    pub damage_dealt: i64,
    pub minion_damage_dealt: i64,
    pub damage_taken: i64,
    pub highest_damage_dealt: i64,

    // healing
    pub healing_recieved: i64,

    // progression
    pub highest_level: i32,
    pub gold_collected: i64,
    pub gold_spent: i64,
    pub lunar_coins_spent: i64,
    pub purchases: i32,
    pub blood_purchases: i32,

    // movement
    pub distance_traveled_metres: i64,
}

impl TryFrom<RunReportDTO> for run_report::ActiveModel {
    type Error = Box<dyn Error>;
    fn try_from(dto: RunReportDTO) -> Result<Self, Self::Error> {
        let score = total_run_score(&dto);
        Ok(Self {
            upload_time: Set(chrono::Utc::now().naive_utc()),

            survivor: Set(dto.survivor),
            ending: Set(dto.ending),
            start_time: Set(dto
                .start_time
                .parse::<DateTime<Utc>>()
                .expect("invalid start time format")
                .naive_utc()),
            difficulty: Set(dto.difficulty),
            time_alive_seconds: Set(dto.time_alive_seconds),
            artifacts: Set(dto.artifacts),
            stages_completed: Set(dto.stages_completed),
            score: Set(score),
            items: Set(dto.items),
            items_collected: Set(dto.items_collected),
            drones_purchased: Set(dto.drones_purchased),
            turrets_purchased: Set(dto.turrets_purchased),
            kills: Set(dto.kills),
            elite_kills: Set(dto.elite_kills),
            minion_kills: Set(dto.minion_kills),
            deaths: Set(dto.deaths),
            damage_dealt: Set(dto.damage_dealt),
            minion_damage_dealt: Set(dto.minion_damage_dealt),
            damage_taken: Set(dto.damage_taken),
            highest_damage_dealt: Set(dto.highest_damage_dealt),
            healing_recieved: Set(dto.healing_recieved),
            highest_level: Set(dto.highest_level),
            gold_collected: Set(dto.gold_collected),
            gold_spent: Set(dto.gold_spent),
            lunar_coins_spent: Set(dto.lunar_coins_spent),
            purchases: Set(dto.purchases),
            blood_purchases: Set(dto.blood_purchases),
            distance_traveled_metres: Set(dto.distance_traveled_metres),

            ..Default::default()
        })
    }
}

fn total_run_score(dto: &RunReportDTO) -> i64 {
    let s = ScoringTable::new();
    let score = dto.time_alive_seconds as f32 * s.time_alive_seconds
        + dto.kills as f32 * s.kills
        + dto.minion_kills as f32 * s.minion_kills
        + dto.damage_dealt as f32 * s.damage_dealt
        + dto.minion_damage_dealt as f32 * s.minion_damage_dealt
        + dto.highest_damage_dealt as f32 * s.highest_damage_dealt
        + dto.highest_level as f32 * s.highest_level
        + dto.gold_collected as f32 * s.gold_collected
        + dto.items_collected as f32 * s.items_collected
        + dto.stages_completed as f32 * s.stages_completed
        + dto.purchases as f32 * s.purchases;
    score as i64
}
