#[macro_use]
extern crate rocket;

use rocket::State;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set};
use entity::game_run;

mod db;
mod entity;

#[get("/")]
async fn index(_conn: &State<DatabaseConnection>) -> String {
    "hello".into()
}

#[post("/game-run/<name>")]
async fn create_game_run(db: &State<DatabaseConnection>, name: String) -> String {
    let game_run = game_run::ActiveModel {
        name: Set(name),
        created_at: Set(chrono::Utc::now().naive_utc()),
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

    rocket::build().manage(db).mount("/", routes![index, create_game_run])
}
