use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    auth_session::WISession,
    entity::run_report,
    error::{make_error, WIError},
    run_report_dto::RunReportDTO,
    WIState,
};

pub async fn post(
    session: WISession,
    State(state): State<Arc<WIState>>,
    Json(payload): Json<RunReportDTO>,
) -> Result<Json<&'static str>, WIError> {
    let mut game_run: run_report::ActiveModel =
        payload
            .try_into()
            .map_err(|e: Box<dyn std::error::Error>| {
                make_error(StatusCode::BAD_REQUEST, format!("Failed to parse: {e}"))
            })?;
    game_run.user_id = Set(session.0.user.id);
    game_run.insert(&state.db).await.map_err(|e| {
        make_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert run: {e}"),
        )
    })?;
    Ok(Json("Game run created"))
}
