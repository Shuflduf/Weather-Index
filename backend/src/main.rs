#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{entity::run_report, run_report_dto::RunReportDTO};

mod db;
mod entity;
mod run_report_dto;

#[get("/")]
async fn index(_conn: &State<DatabaseConnection>) -> String {
    "hello".into()
}

#[post("/new-run", data = "<data>")]
async fn create_game_run(db: &State<DatabaseConnection>, data: Json<RunReportDTO>) -> String {
    let game_run: run_report::ActiveModel = data.into_inner().try_into().unwrap();
    match game_run.insert(db.inner()).await {
        Ok(_) => "Game run created!".into(),
        Err(e) => format!("Error: {}", e),
    }
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");

    rocket::build()
        .manage(db)
        .mount("/", routes![index, create_game_run])
}
