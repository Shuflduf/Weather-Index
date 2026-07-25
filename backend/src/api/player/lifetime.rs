use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde::Serialize;

use crate::{
    api::global::{aggregate_stats, GlobalStatsDTO},
    entity::{run_report, user},
    error::{db_error, make_error, WIError},
    WIState,
};

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<GlobalStatsDTO>, WIError> {
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

    // TODO: also send avg. or make it part of global/
    let stats = aggregate_stats(&state.db, false, Some(&player.id)).await?;

    Ok(Json(stats.into()))
}
