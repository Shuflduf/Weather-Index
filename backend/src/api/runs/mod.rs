use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter,
};
use serde::Serialize;

use crate::{
    entity::{
        run_report::{self, Entity as RunReport},
        user,
    },
    error::{make_error, WIError},
    WIState,
};

pub(crate) mod list;
pub(crate) mod new;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunReportWithUser {
    #[serde(flatten)]
    report: serde_json::Value,
    user_image: Option<String>,
    user_username: Option<String>,
    user_display_username: Option<String>,
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
        report: serde_json::to_value(report).unwrap(),
        user_username: user.as_ref().and_then(|u| u.username.clone()),
        user_display_username: user.as_ref().and_then(|u| u.display_username.clone()),
        user_image: user.as_ref().and_then(|u| u.image.clone()),
    };
    Ok(Json(result))
}
