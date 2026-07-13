use std::sync::Arc;

use better_auth::BetterAuth;
use sea_orm::DatabaseConnection;

use crate::auth_entities::AppAdapter;

pub mod api;
pub mod auth_entities;
pub mod auth_extractor;
pub mod db;
pub mod entity;
pub mod error;
pub mod ror2;
pub mod run_report_dto;
pub mod scoring_table;

pub struct WIState {
    pub db: DatabaseConnection,
    pub auth: Arc<BetterAuth<AppAdapter>>,
}
