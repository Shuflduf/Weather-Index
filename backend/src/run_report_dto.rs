use std::error::Error;

use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::entity::run_report::{self, EclipseLevel};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunReportDTO {
    pub survivor: String,
    pub ending: String,
    pub start_time: String,
    pub difficulty: String,
    pub time_alive_seconds: i64,
    pub stages_completed: i64,

    // items
    pub items: serde_json::Value,
    // pub items_collected: i64,

    // drones
    pub drones_purchased: i64,
    pub turrets_purchased: i64,

    // combat
    pub kills: i64,
    pub elite_kills: i64,
    pub minion_kills: i64,
    pub deaths: i64,

    // damage
    pub damage_dealt: i64,
    pub minion_damage_dealt: i64,
    pub damage_taken: i64,
    pub highest_damage_dealt: i64,

    // healing
    pub healing_recieved: i64,

    // progression
    pub highest_level: i64,
    pub gold_collected: i64,
    pub gold_spent: i64,
    pub lunar_coins_spent: i64,
    pub purchases: i64,
    pub blood_purchases: i64,

    // movement
    pub distance_traveled_metres: i64,
}

impl TryFrom<RunReportDTO> for run_report::ActiveModel {
    type Error = Box<dyn Error>;
    fn try_from(dto: RunReportDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            upload_time: Set(chrono::Utc::now().naive_utc()),

            survivor: Set(dto.survivor),
            ending: Set(dto.ending.try_into()?),
            start_time: Set(dto
                .start_time
                .parse::<DateTime<Utc>>()
                .expect("invalid start time format")
                .naive_utc()),
            eclipse_level: Set(parse_eclipse_level(&dto.difficulty)),
            difficulty: Set(dto.difficulty.try_into()?),
            time_alive_seconds: Set(dto.time_alive_seconds),
            stages_completed: Set(dto.stages_completed),
            items: Set(dto.items),
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

impl TryFrom<String> for run_report::Ending {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "MainEnding" => Ok(Self::Won),
            "StandardLoss" => Ok(Self::Lost),
            "LimboEnding" => Ok(Self::Limbo),
            "ObliterationEnding" => Ok(Self::Obliteration),
            e => Err(format!("unknown ending: {e}").into()),
        }
    }
}

impl TryFrom<String> for run_report::Difficulty {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "DIFFICULTY_EASY_NAME" => Ok(Self::Drizzle),
            "DIFFICULTY_NORMAL_NAME" => Ok(Self::Rainstorm),
            "DIFFICULTY_HARD_NAME" => Ok(Self::Monsoon),
            other => {
                if other.starts_with("ECLIPSE") {
                    Ok(Self::Eclipse)
                } else {
                    Err(format!("unknown difficulty: {other}").into())
                }
            }
        }
    }
}

fn parse_eclipse_level(value: &str) -> EclipseLevel {
    if value.starts_with("ECLIPSE") {
        value.split("_").nth(1)?.parse().ok()
    } else {
        None
    }
}
