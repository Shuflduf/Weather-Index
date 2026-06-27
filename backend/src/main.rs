use std::{error::Error, sync::Arc};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{entity::run_report, error::WIError, run_report_dto::RunReportDTO};

mod db;
mod entity;
mod error;
mod run_report_dto;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");
    let state = Arc::new(db);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/new-run", post(insert_new_run))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn insert_new_run(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<RunReportDTO>,
) -> Result<Json<&'static str>, WIError> {
    let game_run: run_report::ActiveModel = payload
        .try_into()
        .map_err(|e: Box<dyn Error>| WIError::Parse(e.to_string()))?;
    game_run
        .insert(&*db)
        .await
        .map_err(|e| WIError::DB(e.to_string()))?;
    Ok(Json("Game run created"))
}
