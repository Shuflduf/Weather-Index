use axum::Json;

fn parse(text: &str) -> Json<serde_json::Value> {
    Json(serde_json::from_str::<serde_json::Value>(text).unwrap())
}

pub async fn items() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/items.json"))
}

pub async fn tiers() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/tiers.json"))
}

pub async fn bodies() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/bodies.json"))
}

pub async fn endings() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/endings.json"))
}

pub async fn scoring() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/scoring.json"))
}

pub async fn artifacts() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/artifacts.json"))
}

pub async fn difficulties() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/difficulties.json"))
}

pub async fn environments() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/environments.json"))
}

pub async fn equipment() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/equipment.json"))
}

pub async fn skills() -> Json<serde_json::Value> {
    parse(include_str!("../../../data/skills.json"))
}

pub async fn mods() -> Json<serde_json::Value> {
    Json(serde_json::json!(
        include_str!("../../../data/vanilla_mods.txt")
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .map(|l| l.trim().to_string())
            .collect::<Vec<_>>()
    ))
}
