use std::error::Error;

use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::{
    data::{ORDERED_DIFFICULTIES, ORDERED_SURVIVORS},
    entity::run_report::{self},
    ror2::{self, endings},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemEvent {
    id: i16,
    count: i32,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentEvent {
    id: i16,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageInteractable {
    name: String,
    time: Option<i32>,
    item: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StageInfo {
    name: String,
    interactables: Vec<StageInteractable>,
}

#[derive(Deserialize, Debug, Clone, Dummy)]
#[serde(rename_all = "camelCase")]
pub struct RunReportDTO {
    #[dummy(
        expr = "ORDERED_SURVIVORS[Faker.fake::<usize>() % ORDERED_SURVIVORS.len()].to_string()"
    )]
    pub survivor: String,

    #[dummy(expr = "vec![]")]
    pub skills: Vec<i32>,
    #[dummy(expr = "endings()[Faker.fake::<usize>() % endings().len()].name.clone()")]
    pub ending: String,
    #[dummy(expr = "chrono::Utc::now().to_rfc3339()")]
    pub start_time: String,
    #[dummy(
        expr = "ORDERED_DIFFICULTIES[Faker.fake::<usize>() % ORDERED_DIFFICULTIES.len()].to_string()"
    )]
    pub difficulty: String,
    #[dummy(faker = "1..1000i64")]
    pub time_alive_seconds: i64,
    #[dummy(expr = "vec![]")]
    pub artifacts: Vec<String>,
    #[dummy(expr = "vec![]")]
    pub mods: Vec<String>,
    #[dummy(faker = "1..1000i16")]
    pub stages_completed: i16,
    #[dummy(expr = "vec![]")]
    pub stage_history: Vec<StageInfo>,

    // items
    #[dummy(expr = "serde_json::json!({})")]
    pub items: serde_json::Value,
    #[dummy(expr = "-1")]
    pub equipment: i32,
    #[dummy(faker = "1..1000i32")]
    pub items_collected: i32,
    #[dummy(expr = "vec![]")]
    pub item_history: Vec<ItemEvent>,
    #[dummy(expr = "vec![]")]
    pub equipment_history: Vec<EquipmentEvent>,

    //
    // drones
    #[dummy(faker = "1..1000i16")]
    pub drones_purchased: i16,
    #[dummy(faker = "1..1000i16")]
    pub turrets_purchased: i16,

    // combat
    #[dummy(faker = "1..1000i32")]
    pub kills: i32,
    #[dummy(faker = "1..1000i32")]
    pub elite_kills: i32,
    #[dummy(faker = "1..1000i32")]
    pub minion_kills: i32,
    #[dummy(faker = "1..1000i32")]
    pub deaths: i32,

    // damage
    #[dummy(faker = "1..1000i64")]
    pub damage_dealt: i64,
    #[dummy(faker = "1..1000i64")]
    pub minion_damage_dealt: i64,
    #[dummy(faker = "1..1000i64")]
    pub damage_taken: i64,
    #[dummy(faker = "1..1000i64")]
    pub highest_damage_dealt: i64,

    // healing
    #[dummy(faker = "1..1000i64")]
    pub healing_recieved: i64,

    // progression
    #[dummy(faker = "1..1000i32")]
    pub highest_level: i32,
    #[dummy(faker = "1..1000i64")]
    pub gold_collected: i64,
    #[dummy(faker = "1..1000i32")]
    pub purchases: i32,
    #[dummy(faker = "1..1000i32")]
    pub gold_purchases: i32,
    #[dummy(faker = "1..1000i32")]
    pub blood_purchases: i32,
    #[dummy(faker = "1..1000i32")]
    pub lunar_purchases: i32,

    // movement
    #[dummy(faker = "1..1000i64")]
    pub distance_traveled: i64,
}

impl TryFrom<RunReportDTO> for run_report::ActiveModel {
    type Error = Box<dyn Error>;
    fn try_from(dto: RunReportDTO) -> Result<Self, Self::Error> {
        let score = total_run_score(&dto);
        Ok(Self {
            upload_time: Set(chrono::Utc::now().naive_utc()),

            survivor: Set(dto.survivor),
            skills: Set(dto.skills),
            ending: Set(dto.ending),
            start_time: Set(dto
                .start_time
                .parse::<DateTime<Utc>>()
                .expect("invalid start time format")
                .naive_utc()),
            difficulty: Set(dto.difficulty),
            time_alive_seconds: Set(dto.time_alive_seconds),
            artifacts: Set(dto.artifacts),
            mods: Set(dto.mods),
            stages_completed: Set(dto.stages_completed),
            stage_history: Set(normalize_json(dto.stage_history)),
            score: Set(score),
            items: Set(dto.items),
            equipment: Set(if dto.equipment == -1 {
                None
            } else {
                Some(dto.equipment)
            }),
            items_collected: Set(dto.items_collected),
            item_history: Set(normalize_json(dto.item_history)),
            equipment_history: Set(normalize_json(dto.equipment_history)),
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
            purchases: Set(dto.purchases),
            gold_purchases: Set(dto.gold_purchases),
            blood_purchases: Set(dto.blood_purchases),
            lunar_purchases: Set(dto.lunar_purchases),
            distance_traveled: Set(dto.distance_traveled),

            ..Default::default()
        })
    }
}

fn total_run_score(dto: &RunReportDTO) -> i64 {
    let s = ror2::scoring_table();
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

fn normalize_json<'a, T: Deserialize<'a> + Serialize>(thing: Vec<T>) -> Vec<serde_json::Value> {
    thing
        .iter()
        .map(|e| serde_json::to_value(e).expect("Cast to serde_json::Value failed"))
        .collect()
}
