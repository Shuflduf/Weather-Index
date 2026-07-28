use std::sync::Arc;

use axum::{extract::State, Json};
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, PaginatorTrait, QueryFilter, QuerySelect};
use serde::Serialize;

use crate::{
    entity::run_report,
    error::{db_error, WIError},
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
pub async fn get(State(state): State<Arc<WIState>>) -> Result<Json<OverallResponse>, WIError> {
    let winning_endings = endings()
        .iter()
        .filter(|e| e.is_win)
        .map(|e| e.name.as_ref())
        .collect::<Vec<&str>>();

    let run_count = run_report::Entity::find()
        .count(&state.db)
        .await
        .map_err(db_error)?;
    let win_count = run_report::Entity::find()
        .filter(run_report::Column::Ending.is_in(winning_endings))
        .count(&state.db)
        .await
        .map_err(db_error)?;
    let playtime_seconds: i64 = run_report::Entity::find()
        .select_only()
        .column_as(
            run_report::Column::TimeAliveSeconds.sum().cast_as("BIGINT"),
            "time_alive_seconds",
        )
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
