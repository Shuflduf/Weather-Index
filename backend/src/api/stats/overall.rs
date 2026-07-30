use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter, QuerySelect};
use serde::Serialize;

use crate::{
    api::{player::find_player, stats::UsernameQuery},
    entity::{run_report, user},
    error::{db_error, make_error, WIError},
    ror2::endings,
    WIState,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallResponse {
    run_count: u64,
    win_count: u64,
    playtime_seconds: i64,
}
pub async fn get(
    State(state): State<Arc<WIState>>,
    params: Query<UsernameQuery>,
) -> Result<Json<OverallResponse>, WIError> {
    let player: Option<String> = match params.username {
        Some(ref username) => Some(find_player(&state.db, username).await?.id),
        None => None,
    };

    let winning_endings = endings()
        .iter()
        .filter(|e| e.is_win)
        .map(|e| e.name.as_ref())
        .collect::<Vec<&str>>();

    let mut run_count = run_report::Entity::find();
    if let Some(ref id) = player {
        run_count = run_count.filter(run_report::Column::UserId.eq(id))
    }
    let run_count = run_count.count(&state.db).await.map_err(db_error)?;

    let mut win_count =
        run_report::Entity::find().filter(run_report::Column::Ending.is_in(winning_endings));
    if let Some(ref id) = player {
        win_count = win_count.filter(run_report::Column::UserId.eq(id))
    }
    let win_count = win_count.count(&state.db).await.map_err(db_error)?;

    let mut playtime_seconds = run_report::Entity::find().select_only().column_as(
        run_report::Column::TimeAliveSeconds.sum().cast_as("BIGINT"),
        "time_alive_seconds",
    );

    if let Some(ref id) = player {
        playtime_seconds = playtime_seconds.filter(run_report::Column::UserId.eq(id))
    }

    let playtime_seconds = playtime_seconds
        .into_tuple()
        .one(&state.db)
        .await
        .map_err(db_error)?
        .unwrap_or_default();

    Ok(Json(OverallResponse {
        run_count,
        win_count,
        playtime_seconds,
    }))
}
