#[macro_use]
extern crate rocket;

use entity::game_run;
use rocket::{serde::json::Json, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

mod db;
mod entity;

#[get("/")]
async fn index(_conn: &State<DatabaseConnection>) -> String {
    "hello".into()
}

#[post("/new-run", data = "<data>")]
async fn create_game_run(db: &State<DatabaseConnection>, data: Json<game_run::Model>) -> String {
    let game_run = game_run::ActiveModel {
        end_state: Set(data.end_state.clone()),
        upload_time: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

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
