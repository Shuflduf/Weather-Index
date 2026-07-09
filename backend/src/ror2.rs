use std::sync::OnceLock;

use serde::Deserialize;

static ITEMS: OnceLock<Vec<Item>> = OnceLock::new();
static SCORING: OnceLock<ScoringTable> = OnceLock::new();
static TIERS: OnceLock<Vec<Tier>> = OnceLock::new();
static ENDINGS: OnceLock<Vec<Ending>> = OnceLock::new();

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    id: i32,
    name: String,
    name_token: String,
    display_name: String,
    tier: Option<String>,
    helper: bool,
    icon: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    name: String,
    sort: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ending {
    pub name: String,
    pub name_token: String,
    pub ending_message: String,
    pub display_name: String,
    pub is_win: bool,
    pub icon: String,
    pub color_fg: String,
    pub color_bg: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScoringTable {
    pub time_alive_seconds: f32,
    pub kills: f32,
    pub minion_kills: f32,
    pub damage_dealt: f32,
    pub minion_damage_dealt: f32,
    pub highest_damage_dealt: f32,
    pub highest_level: f32,
    pub gold_collected: f32,
    pub items_collected: f32,
    pub stages_completed: f32,
    pub purchases: f32,
}

pub fn items() -> &'static Vec<Item> {
    ITEMS.get_or_init(|| serde_json::from_str(include_str!("data/items.json")).unwrap())
}

pub fn scoring_table() -> &'static ScoringTable {
    SCORING.get_or_init(|| serde_json::from_str(include_str!("data/scoring.json")).unwrap())
}

pub fn tiers() -> &'static Vec<Tier> {
    TIERS.get_or_init(|| serde_json::from_str(include_str!("data/tiers.json")).unwrap())
}

pub fn endings() -> &'static Vec<Ending> {
    ENDINGS.get_or_init(|| serde_json::from_str(include_str!("data/endings.json")).unwrap())
}
