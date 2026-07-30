use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    api::{player::find_player, stats::UsernameQuery},
    entity::run_report,
    error::{db_error, WIError},
    WIState,
};

pub async fn get(
    State(state): State<Arc<WIState>>,
    params: Query<UsernameQuery>,
) -> Result<Json<serde_json::Value>, WIError> {
    let mut query = run_report::Entity::find();

    if let Some(ref username) = params.username {
        let player = find_player(&state.db, username).await?.id;
        query = query.filter(run_report::Column::UserId.eq(player));
    }

    let runs = query.all(&state.db).await.map_err(db_error)?;
    let mut freq: HashMap<String, i64> = HashMap::new();
    for run in runs {
        *freq.entry(run.survivor).or_insert(0) += 1;
    }

    Ok(Json(serde_json::to_value(freq).unwrap()))
}
