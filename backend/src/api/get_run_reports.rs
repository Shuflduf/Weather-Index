use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde_json::json;

use crate::{
    entity::run_report::{self, Entity as RunReport},
    error::{make_error, WIError},
    WIState,
};

pub async fn get(
    State(state): State<Arc<WIState>>,
) -> Result<Json<Vec<run_report::Model>>, WIError> {
    let reports = RunReport::find().all(&state.db).await.map_err(|e| {
        make_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to query runs: {e}"),
        )
    })?;
    Ok(Json(reports))
}
