use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "deviceCode")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "deviceCode")]
    pub device_code: String,
    #[sea_orm(column_name = "userCode")]
    pub user_code: String,
    #[sea_orm(column_name = "userId")]
    pub user_id: Option<String>,
    #[sea_orm(column_name = "clientId")]
    pub client_id: Option<String>,
    #[sea_orm(column_name = "scope")]
    pub scope: Option<String>,
    #[sea_orm(column_name = "status")]
    pub status: String,
    #[sea_orm(column_name = "expiresAt")]
    pub expires_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "lastPolledAt")]
    pub last_polled_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_name = "pollingInterval")]
    pub polling_interval: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
