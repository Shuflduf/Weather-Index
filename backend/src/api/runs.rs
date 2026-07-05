use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    entity::{
        run_report::{self, Entity as RunReport},
        user,
    },
    error::{make_error, WIError},
    WIState,
};

#[derive(Serialize)]
pub struct RunReportWithUser {
    #[serde(flatten)]
    report: run_report::Model,
    user_image: Option<String>,
    user_username: Option<String>,
}

pub async fn list(
    State(state): State<Arc<WIState>>,
) -> Result<Json<Vec<RunReportWithUser>>, WIError> {
    let reports = RunReport::find()
        .find_also_related(user::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query runs: {e}"),
            )
        })?;

    let results = reports
        .into_iter()
        .map(|(report, user)| RunReportWithUser {
            report,
            user_username: user.as_ref().and_then(|u| u.username.clone()),
            user_image: user.as_ref().and_then(|u| u.image.clone()),
        })
        .collect();
    Ok(Json(results))
}

pub async fn get(
    Path(id): Path<i32>,
    State(state): State<Arc<WIState>>,
) -> Result<Json<RunReportWithUser>, WIError> {
    let (report, user) = RunReport::find()
        .filter(run_report::Column::Id.eq(id))
        .find_also_related(user::Entity)
        .one(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query runs: {e}"),
            )
        })?
        .ok_or_else(|| make_error(StatusCode::NOT_FOUND, format!("Run {id} not found")))?;

    let result = RunReportWithUser {
        report,
        user_username: user.as_ref().and_then(|u| u.username.clone()),
        user_image: user.as_ref().and_then(|u| u.image.clone()),
    };
    Ok(Json(result))
}
