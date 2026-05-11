#[macro_use]
extern crate rocket;

use rocket::State;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbConn};

mod db;

#[get("/")]
async fn index(conn: &State<DatabaseConnection>) -> String {
    "hello".into()
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");

    rocket::build().manage(db).mount("/", routes![index])
}
