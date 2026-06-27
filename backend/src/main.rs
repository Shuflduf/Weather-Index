use std::{env, error::Error, sync::Arc};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use better_auth::{
    plugins::{
        oauth::OAuthProvider, AccountManagementPlugin, EmailPasswordPlugin, OAuthPlugin,
        PasswordManagementPlugin, SessionManagementPlugin,
    },
    AuthConfig, AxumIntegration, BetterAuth, MemoryDatabaseAdapter,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{entity::run_report, error::WIError, run_report_dto::RunReportDTO};

mod auth_entities;
mod db;
mod entity;
mod error;
mod run_report_dto;

struct WIState {
    db: DatabaseConnection,
    auth: Arc<BetterAuth<MemoryDatabaseAdapter>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");

    let auth_config =
        AuthConfig::new(env::var("ENCRYPTION_KEY")?).base_url("http://localhost:3000/auth");
    let auth = Arc::new(
        BetterAuth::<MemoryDatabaseAdapter>::new(auth_config)
            .database(MemoryDatabaseAdapter::new())
            .plugin(EmailPasswordPlugin::new())
            .plugin(SessionManagementPlugin::new())
            .plugin(PasswordManagementPlugin::new())
            .plugin(AccountManagementPlugin::new())
            .plugin(OAuthPlugin::new().add_provider(
                "github",
                OAuthProvider::github(&env::var("GITHUB_ID")?, &env::var("GITHUB_SECRET")?),
            ))
            .build()
            .await?,
    );
    let auth_router = auth.clone().axum_router().with_state(auth.clone());

    let state = Arc::new(WIState {
        db,
        auth: auth.clone(),
    });

    let app = Router::new()
        .nest("/auth", auth_router)
        .route("/", get(|| async { "Hello, World!" }))
        .route("/new-run", post(insert_new_run))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn insert_new_run(
    State(state): State<Arc<WIState>>,
    Json(payload): Json<RunReportDTO>,
) -> Result<Json<&'static str>, WIError> {
    let game_run: run_report::ActiveModel = payload
        .try_into()
        .map_err(|e: Box<dyn Error>| WIError::Parse(e.to_string()))?;
    game_run
        .insert(&state.db)
        .await
        .map_err(|e| WIError::DB(e.to_string()))?;
    Ok(Json("Game run created"))
}
