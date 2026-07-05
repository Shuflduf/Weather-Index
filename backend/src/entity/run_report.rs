use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "run_reports")]
pub struct Model {
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    #[serde(default)]
    pub upload_time: DateTime,

    // run info
    pub survivor: String,
    pub start_time: DateTime,
    pub ending: String,
    pub difficulty: String,
    pub time_alive_seconds: i64,
    pub stages_completed: i16,
    pub score: i64,
    // pub run_time_seconds: u32,

    // items
    pub items: serde_json::Value,
    pub items_collected: i32,
    // pub items_scrapped: u16,
    // pub chests_left_behind: u16,

    // drones
    pub drones_purchased: i16,
    pub turrets_purchased: i16,
    // pub drones_scrapped: u16,
    // pub drone_deaths: u32,
    // pub drones_left_behind: u16,

    // combat
    pub kills: i32,
    pub elite_kills: i32,
    pub minion_kills: i32,
    pub deaths: i32,

    // damage
    pub damage_dealt: i64,
    pub minion_damage_dealt: i64,
    // pub minion_damage_taken: i64,
    pub damage_taken: i64,
    // pub damage_dealt_over_time: i64,
    pub highest_damage_dealt: i64,
    // pub damage_blocked: u32,

    // healing
    pub healing_recieved: i64,
    // pub minion_healing_recieved: i64,

    // progression
    pub highest_level: i32,
    pub gold_collected: i64,
    pub gold_spent: i64,
    pub lunar_coins_spent: i64,
    pub purchases: i32,
    pub blood_purchases: i32,

    // movement
    pub distance_traveled_metres: i64,
    // pub jumps: u32,

    // abilities
    // pub equipment_activations: u16,
    // pub skill_activations: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        None
    }
}
