use sea_orm::entity::prelude::*;
use serde::Deserialize;

pub type EclipseLevel = Option<u8>;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, DeriveActiveEnum, EnumIter)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "PascalCase"
)]
pub enum Ending {
    Won,
    Lost,
    Limbo,
    Obliteration,
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
#[sea_orm(table_name = "run_reports")]
pub struct Model {
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i32,
    #[serde(default)]
    pub upload_time: DateTime,

    // run info
    pub survivor: String,
    pub start_time: DateTime,
    pub ending: Ending,
    pub difficulty: Difficulty,
    pub eclipse_level: Option<u8>, // cant be EclipseLevel for sea orm reasons 🐸🚀
    pub time_alive_seconds: u64,
    pub stages_completed: u64,
    // pub run_time_seconds: u32,

    // items
    pub items_collected: u64,
    // pub items_scrapped: u16,
    // pub chests_left_behind: u16,

    // drones
    pub drones_purchased: u64,
    pub turrets_purchased: u64,
    // pub drones_scrapped: u16,
    // pub drone_deaths: u32,
    // pub drones_left_behind: u16,

    // combat
    pub kills: u64,
    pub elite_kills: u64,
    pub minion_kills: u64,
    pub deaths: u64,

    // damage
    pub damage_dealt: u64,
    pub minion_damage_dealt: u64,
    // pub minion_damage_taken: u64,
    pub damage_taken: u64,
    // pub damage_dealt_over_time: u64,
    pub highest_damage_dealt: u64,
    // pub damage_blocked: u32,

    // healing
    pub healing_recieved: u64,
    // pub minion_healing_recieved: u64,

    // progression
    pub highest_level: u64,
    pub gold_collected: u64,
    pub gold_spent: u64,
    pub lunar_coins_spent: u64,
    pub purchases: u64,
    pub blood_purchases: u64,

    // movement
    pub distance_traveled_metres: u64,
    // pub jumps: u32,

    // abilities
    // pub equipment_activations: u16,
    // pub skill_activations: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
