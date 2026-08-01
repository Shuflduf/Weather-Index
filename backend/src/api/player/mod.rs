use std::{collections::HashMap, env, sync::Arc};

use axum::{
    extract::{Path, State},
    response::Redirect,
    Form, Json,
};
use reqwest::StatusCode;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    auth_session::WISession,
    entity::{run_report, user},
    error::{db_error, make_error, WIError},
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

pub async fn find_player(db: &DatabaseConnection, username: &str) -> Result<user::Model, WIError> {
    user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
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
                format!("Player `{username}` not found"),
            )
        })
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<PlayerGetResponse>, WIError> {
    let player = find_player(&state.db, &username).await?;
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
    let run_count = runs.len() as i32;
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlayer {
    image: Option<String>,
    username: Option<String>,
    display_username: Option<String>,
    about_me: Option<String>,
    region: Option<String>,
}

pub async fn update(
    session: WISession,
    State(state): State<Arc<WIState>>,
    Json(payload): Json<UpdatePlayer>,
) -> Result<(), WIError> {
    let new_user = user::ActiveModel {
        id: Set(session.0.user.id),
        username: Set(payload.username),
        display_username: Set(payload.display_username),
        image: Set(payload.image),
        region: Set(payload.region),
        about_me: Set(payload.about_me),
        ..Default::default()
    };
    user::Entity::update(new_user)
        .validate()
        .map_err(|e| {
            make_error(
                StatusCode::BAD_REQUEST,
                format!("Could not validate data: {e}"),
            )
        })?
        .exec(&state.db)
        .await
        .map_err(db_error)?;

    // Ok(Redirect::permanent(&format!(
    //     "{}/settings",
    //     env::var("FRONTEND_URL")
    //         .map_err(|e| make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    // )))
    //
    Ok(())
}
