fn compress(json: &str) -> String {
    serde_json::from_str::<serde_json::Value>(json)
        .unwrap()
        .to_string()
}

pub async fn items() -> String {
    compress(include_str!("../../../data/items.json"))
}

pub async fn tiers() -> String {
    compress(include_str!("../../../data/tiers.json"))
}

pub async fn bodies() -> String {
    compress(include_str!("../../../data/bodies.json"))
}

pub async fn endings() -> String {
    compress(include_str!("../../../data/endings.json"))
}

pub async fn scoring() -> String {
    compress(include_str!("../../../data/scoring.json"))
}

pub async fn artifacts() -> String {
    compress(include_str!("../../../data/artifacts.json"))
}

pub async fn difficulties() -> String {
    compress(include_str!("../../../data/difficulties.json"))
}

pub async fn environments() -> String {
    compress(include_str!("../../../data/environments.json"))
}
