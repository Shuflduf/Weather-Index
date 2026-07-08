use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    entity::user,
    error::{make_error, WIError},
    WIState,
};

#[derive(Serialize)]
pub struct PlayerInfo {
    pub id: String,
    pub image: Option<String>,
    pub username: Option<String>,
    pub display_username: Option<String>,
    pub about_me: Option<String>,
    pub region: Option<String>,
}

impl From<user::Model> for PlayerInfo {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            image: value.image,
            username: value.username,
            display_username: value.display_username,
            about_me: value.about_me,
            region: value.region,
        }
    }
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Path(username): Path<String>,
) -> Result<Json<PlayerInfo>, WIError> {
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
    Ok(Json(player.into()))
}
