use std::{env, error::Error, sync::Arc};

use axum::{
    body::{self, Body},
    http::{header, HeaderValue, Request, Response},
    middleware::{self, Next},
    routing::{self, get},
    Router,
};
use better_auth::{
    plugins::{
        oauth::OAuthProvider, AccountManagementPlugin, DeviceAuthorizationConfig,
        DeviceAuthorizationPlugin, EmailPasswordPlugin, OAuthPlugin, PasswordManagementPlugin,
        SessionManagementPlugin,
    },
    AuthBuilder, AuthConfig, AxumIntegration, CorsConfig, CsrfConfig, SameSite,
};
use reqwest::{
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
    },
    Method, StatusCode,
};

use tower_http::cors::{AllowHeaders, CorsLayer};
use weather_index::{api, auth_entities::AppAdapter, db, WIState};

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

    let mut auth_config = AuthConfig::new(env::var("ENCRYPTION_KEY")?)
        .trusted_origins(vec![env::var("FRONTEND_URL")?])
        .base_url(format!("{}/auth", env::var("BACKEND_URL")?));
    auth_config.session.cookie_same_site = SameSite::None;

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
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(AllowHeaders::list([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ]))
        .allow_credentials(true);

    let api_router = api::router().layer(cors);
    let app = Router::new()
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .layer(middleware::from_fn(cors_middleware))
        .route("/", get(|| async { "Hello, World!" }))
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

async fn cors_middleware(
    req: Request<body::Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    if req.method() == Method::OPTIONS {
        let cors_config =
            CorsConfig::new().allowed_origin(env::var("FRONTEND_URL").unwrap_or_default());

        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(
                ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
                cors_config.allowed_origins.join(", "),
            )
            .header(
                ACCESS_CONTROL_ALLOW_METHODS.to_string(),
                cors_config.allowed_methods.join(", "),
            )
            .header(
                ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
                cors_config.allowed_headers.join(", "),
            )
            .header(
                ACCESS_CONTROL_ALLOW_CREDENTIALS.to_string(),
                cors_config.allow_credentials.to_string(),
            )
            .header(ACCESS_CONTROL_MAX_AGE, cors_config.max_age)
            .body(Body::empty())
            .unwrap());
    }
    Ok(next.run(req).await)
}
