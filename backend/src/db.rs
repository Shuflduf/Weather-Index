use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, error::Error};

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect(ConnectOptions::new(env::var("DATABASE_URL")?)).await?;

    db.get_schema_registry("weather_index::entity::*")
        .sync(&db)
        .await?;

    Ok(db)
}
