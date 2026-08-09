use std::{env, sync::Arc};

use better_auth::BetterAuth;
use reqwest::StatusCode;
use sea_orm::DatabaseConnection;

use crate::{
    auth_entities::{AppAdapter, AppHookedAdapter},
    error::{make_error, WIError},
};

pub mod api;
pub mod auth_entities;
pub mod auth_hooks;
pub mod auth_session;
pub mod data;
pub mod db;
pub mod entity;
pub mod error;
pub mod ror2;
pub mod run_report_dto;
pub mod scoring_table;
pub mod slack_oauth;

pub struct WIState {
    pub db: DatabaseConnection,
    pub auth: Arc<BetterAuth<AppHookedAdapter>>,
}

pub fn get_var(key: &str) -> Result<String, WIError> {
    env::var(key).map_err(|e| make_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
