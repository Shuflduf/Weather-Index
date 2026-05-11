use sea_orm::{Database, DatabaseConnection};
use std::{env, error::Error};

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect(env::var("DATABASE_URL")?).await?;
    
    // Sync schema with entity definitions
    db.get_schema_registry("rordb_backend::entity::*")
        .sync(&db)
        .await?;
    
    Ok(db)
}
