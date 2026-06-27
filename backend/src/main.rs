use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{sea_query::prelude::serde_json, ActiveModelTrait, DatabaseConnection};

use crate::{entity::run_report, run_report_dto::RunReportDTO};

mod db;
mod entity;
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
) -> String {
    let game_run: run_report::ActiveModel = payload.try_into().unwrap();
    match game_run.insert(&*db).await {
        Ok(_) => "Game run created!".into(),
        Err(e) => format!("Error: {}", e),
    }
}
