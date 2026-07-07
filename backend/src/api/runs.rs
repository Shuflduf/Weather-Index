use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
pub struct ListParams {
    #[serde(default = "default_sort_by")]
    by: String,
    #[serde(default = "default_sort")]
    sort: String,
}

fn default_sort_by() -> String {
    "id".to_string()
}
fn default_sort() -> String {
    "DESC".to_string()
}

pub async fn list(
    State(state): State<Arc<WIState>>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<RunReportWithUser>>, WIError> {
    println!("{params:?}");
    let order = match params.sort.as_ref() {
        "DESC" => Order::Desc,
        "ASC" => Order::Asc,
        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            "field `order` should be either `DESC` or `ASC`".into(),
        ))?,
    };
    let sort_by = match params.by.as_ref() {
        "id" => run_report::Column::Id,
        "difficulty" => run_report::Column::Difficulty,
        "survivor" => run_report::Column::Survivor,
        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            format!("{} is an invalid column name", params.by),
            // TODO: show actual column names to sort by
        ))?,
    };

    let reports = RunReport::find()
        .order_by(sort_by, order)
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
