use std::{env, error::Error, sync::Arc};

use axum::{
    body,
    extract::State,
    http::{header, HeaderValue, Request, Response},
    middleware::{self, Next},
    routing::{get, post},
    Json, Router,
};
use better_auth::{
    plugins::{
        oauth::OAuthProvider, AccountManagementPlugin, DeviceAuthorizationConfig,
        DeviceAuthorizationPlugin, EmailPasswordPlugin, OAuthPlugin, PasswordManagementPlugin,
        SessionManagementPlugin,
    },
    AuthBuilder, AuthConfig, AxumIntegration, CsrfConfig,
};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use sqlx::Any;
use tower_http::cors::{self, CorsLayer};
use weather_index::{
    api,
    auth_entities::AppAdapter,
    auth_session::WISession,
    db,
    entity::run_report,
    error::{make_error, WIError},
    run_report_dto::RunReportDTO,
    WIState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "weather_index=debug,better_auth=debug,better_auth_core=debug,warn",
        ))
        .init();
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");
    let pg_pool = db.get_postgres_connection_pool();

    let adapter = AppAdapter::from_pool(pg_pool.clone());

    let auth_config = AuthConfig::new(env::var("ENCRYPTION_KEY")?)
        .trusted_origins(vec![env::var("FRONTEND_URL")?])
        .base_url(format!("{}/auth", env::var("BACKEND_URL")?));

    let auth = Arc::new(
        AuthBuilder::new(auth_config)
            .csrf(CsrfConfig::new().enabled(false))
            .database(adapter)
            .plugin(EmailPasswordPlugin::new())
            .plugin(PasswordManagementPlugin::new())
            .plugin(SessionManagementPlugin::new())
            .plugin(AccountManagementPlugin::new())
            .plugin(
                OAuthPlugin::new()
                    .add_provider(
                        "github",
                        OAuthProvider::github(&env::var("GITHUB_ID")?, &env::var("GITHUB_SECRET")?),
                    )
                    .add_provider(
                        "discord",
                        OAuthProvider::discord(
                            &env::var("DISCORD_ID")?,
                            &env::var("DISCORD_SECRET")?,
                        ),
                    )
                    .add_provider(
                        "google",
                        OAuthProvider::google(&env::var("GOOGLE_ID")?, &env::var("GOOGLE_SECRET")?),
                    ),
            )
            .plugin(DeviceAuthorizationPlugin::with_config(
                DeviceAuthorizationConfig {
                    enabled: true,
                    verification_uri: format!("{}/device", env::var("FRONTEND_URL")?),
                    interval: 5,
                    expires_in: 1800,
                },
            ))
            .build()
            .await?,
    );
    let auth_router = auth
        .clone()
        .axum_router()
        .with_state(auth.clone())
        .layer(middleware::from_fn(oauth_redirect_middleware));

    let state = Arc::new(WIState {
        db,
        auth: auth.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(env::var("FRONTEND_URL")?.parse::<HeaderValue>()?)
        .allow_headers(cors::Any)
        .allow_methods(cors::Any);

    let app = Router::new()
        .nest("/auth", auth_router)
        // .route("/auth/callback/github", get(github_oauth::handle_callback))
        // .route(
        //     "/auth/callback/discord",
        //     get(discord_oauth::handle_callback),
        // )
        .route("/", get(|| async { "Hello, World!" }))
        .route("/new-run", post(insert_new_run))
        .nest("/api", api::router())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Started");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn oauth_redirect_middleware(
    req: Request<body::Body>,
    next: Next,
) -> Result<Response<body::Body>, StatusCode> {
    let path = req.uri().path().to_string();
    let response = next.run(req).await;

    if path.starts_with("/callback") {
        let frontend_url = env::var("FRONTEND_URL").unwrap();
        let (mut parts, _) = response.into_parts();
        parts.status = StatusCode::FOUND;
        parts
            .headers
            .insert(header::LOCATION, frontend_url.parse().unwrap());
        parts.headers.remove(header::CONTENT_TYPE);

        Ok(Response::from_parts(parts, ().into()))
    } else {
        Ok(response)
    }
}

async fn insert_new_run(
    session: WISession,
    State(state): State<Arc<WIState>>,
    Json(payload): Json<RunReportDTO>,
) -> Result<Json<&'static str>, WIError> {
    let mut game_run: run_report::ActiveModel =
        payload.try_into().map_err(|e: Box<dyn Error>| {
            make_error(StatusCode::BAD_REQUEST, format!("Failed to parse: {e}"))
        })?;
    game_run.user_id = Set(session.0.user.id);
    game_run.insert(&state.db).await.map_err(|e| {
        make_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert run: {e}"),
        )
    })?;
    Ok(Json("Game run created"))
}
