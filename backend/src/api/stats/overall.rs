use std::sync::Arc;

use axum::{extract::State, Json};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
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
}
pub async fn get(State(state): State<Arc<WIState>>) -> Result<Json<OverallResponse>, WIError> {
    let winning_endings = endings()
        .iter()
        .filter(|e| e.is_win)
        .map(|e| e.name.clone())
        .collect::<Vec<String>>();

    let run_count = run_report::Entity::find()
        .count(&state.db)
        .await
        .map_err(db_error)?;
    let win_count = run_report::Entity::find()
        .filter(run_report::Column::Ending.is_in(winning_endings))
        .count(&state.db)
        .await
        .map_err(db_error)?;

    Ok(Json(OverallResponse {
        run_count,
        win_count,
    }))
}
