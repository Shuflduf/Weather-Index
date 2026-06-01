use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, DeriveActiveEnum, EnumIter)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "PascalCase"
)]
pub enum EndState {
    Won,
    Lost,
    Obliterated,
    SuccumbedToTheVoid,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, DeriveActiveEnum, EnumIter)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "PascalCase"
)]
pub enum Difficulty {
    Drizzle,
    Rainstorm,
    Monsoon,
    Eclipse,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "game_run")]
pub struct Model {
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i32,
    #[serde(default)]
    pub upload_time: DateTime,

    // run info
    pub survivor: String,
    pub end_time: DateTime,
    pub end_state: EndState,
    pub difficuly: Difficulty,
    pub eclipse_level: Option<u8>,
    pub time_alive_seconds: u32,
    pub run_time_seconds: u32,
    pub stages_complete: u16,

    // items
    pub items_picked_up: u16,
    pub items_scrapped: u16,
    pub chests_left_behind: u16,

    // drones
    pub drones_repaired: u16,
    pub drones_scrapped: u16,
    pub drone_deaths: u32,
    pub drones_left_behind: u16,

    // combat
    pub kills: u32,
    pub minion_kills: u16,
    pub deaths: u32,

    // damage
    pub damage_dealt: u64,
    pub minion_damage_dealt: u64,
    pub damage_taken: u64,
    pub minion_damage_taken: u64,
    pub damage_dealt_over_time: u64,
    pub highest_damage_hit: u64,
    pub damage_blocked: u32,

    // healing
    pub healing_recieved: u64,
    pub minion_healing_recieved: u64,

    // progression
    pub purchases: u16,
    pub gold_collected: u64,
    pub lunar_coins_spent: u32,
    pub blood_purchases: u16,
    pub highest_level: u16,

    // movement
    pub distance_traveled_metres: u32,
    pub jumps: u32,

    // abilities
    pub equipment_activations: u16,
    pub skill_activations: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
