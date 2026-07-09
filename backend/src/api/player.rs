use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    entity::{run_report, user},
    error::{make_error, WIError},
    ror2, WIState,
};

#[derive(Serialize)]
pub struct PlayerInfo {
    id: String,
    image: Option<String>,
    username: Option<String>,
    display_username: Option<String>,
    about_me: Option<String>,
    region: Option<String>,
}

impl From<user::Model> for PlayerInfo {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            image: value.image,
            username: value.username,
            display_username: value.display_username,
            about_me: value.about_me,
            region: value.region,
        }
    }
}

#[derive(Serialize)]
pub struct PlayerGetResponse {
    #[serde(flatten)]
    player: PlayerInfo,
    run_count: i32,
    win_count: i32,
    favourite_survivor: Option<String>,
    favourite_difficulty: Option<String>,
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<PlayerGetResponse>, WIError> {
    let player = user::Entity::find()
        .filter(user::Column::Username.eq(&username))
        .one(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query users: {e}"),
            )
        })?
        .ok_or_else(|| {
            make_error(
                StatusCode::NOT_FOUND,
                format!("User `{username}` not found"),
            )
        })?;
    let runs = run_report::Entity::find()
        .filter(run_report::Column::UserId.eq(&player.id))
        .all(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query users: {e}"),
            )
        })?;
    let run_count = runs.iter().count() as i32;
    let win_count = runs
        .iter()
        .filter(|run| {
            ror2::endings()
                .iter()
                .find(|e| e.name == run.ending)
                .expect("invalid run")
                .is_win
        })
        .count() as i32;
    let favourite_survivor = most_frequent(
        &runs
            .iter()
            .map(|r| r.survivor.clone())
            .collect::<Vec<String>>(),
    );
    let favourite_difficulty = most_frequent(
        &runs
            .iter()
            .map(|r| r.difficulty.clone())
            .collect::<Vec<String>>(),
    );
    Ok(Json(PlayerGetResponse {
        player: player.into(),
        run_count,
        win_count,
        favourite_survivor,
        favourite_difficulty,
    }))
}

fn most_frequent(items: &[String]) -> Option<String> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(item, _)| item.clone())
}
