use std::error::Error;

use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;

use crate::entity::run_report::{self};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunReportDTO {
    pub survivor: String,
    pub ending: String,
    pub start_time: String,
    pub difficulty: String,
    pub time_alive_seconds: u64,
    pub stages_completed: u64,

    // items
    pub items_collected: u64,

    // drones
    pub drones_purchased: u64,
    pub turrets_purchased: u64,

    // combat
    pub kills: u64,
    pub elite_kills: u64,
    pub minion_kills: u64,
    pub deaths: u64,

    // damage
    pub damage_dealt: u64,
    pub minion_damage_dealt: u64,
    pub damage_taken: u64,
    pub highest_damage_dealt: u64,

    // healing
    pub healing_recieved: u64,

    // progression
    pub highest_level: u64,
    pub gold_collected: u64,
    pub gold_spent: u64,
    pub lunar_coins_spent: u64,
    pub purchases: u64,
    pub blood_purchases: u64,

    // movement
    pub distance_traveled_metres: u64,
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
            ..Default::default()
        })
    }
}

impl TryFrom<String> for run_report::Ending {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "MainEnding" => Ok(Self::Won),
            "StandardLoss" => Ok(Self::Lost),
            "LimboEnding" => Ok(Self::Limbo),
            "ObliterationEnding" => Ok(Self::Obliteration),
            e => Err(format!("unknown ending: {e}")),
        }
    }
}
