use serde::Deserialize;

#[derive(Deserialize)]
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

impl ScoringTable {
    pub fn new() -> Self {
        serde_json::from_str(include_str!("scoring.json")).unwrap()
    }
}
