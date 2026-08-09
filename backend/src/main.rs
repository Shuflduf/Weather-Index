use std::sync::Arc;

use axum::{
    body::{self, Body},
    http::{header, Request, Response},
    middleware::{self, Next},
    routing::get,
    Router,
};
use better_auth::{
    plugins::{
        oauth::OAuthProvider, AccountManagementPlugin, DeviceAuthorizationConfig,
        DeviceAuthorizationPlugin, EmailPasswordPlugin, OAuthPlugin, PasswordManagementPlugin,
        SessionManagementPlugin,
    },
    AuthBuilder, AuthConfig, AxumIntegration, CorsConfig, CsrfConfig, HookedDatabaseAdapter,
    SameSite,
};
use reqwest::{
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
    },
    Method, StatusCode,
};

use weather_index::{
    api::{self},
    auth_entities::AppAdapter,
    auth_hooks::RandomUsernameHook,
    db,
    error::{make_error, WIError},
    get_var, slack_oauth, WIState,
};

#[tokio::main]
async fn main() -> Result<(), WIError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "weather_index=debug,better_auth=debug,better_auth_core=debug,warn",
        ))
        .init();
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");
    let pg_pool = db.get_postgres_connection_pool();

    let adapter = Arc::new(AppAdapter::from_pool(pg_pool.clone()));
    let hooked = HookedDatabaseAdapter::new(adapter.clone())
        .with_hook(Arc::new(RandomUsernameHook::new(adapter.clone())));

    let mut auth_config = AuthConfig::new(get_var("ENCRYPTION_KEY")?)
        .trusted_origins(vec![get_var("FRONTEND_URL")?])
        .base_url(format!("{}/auth", get_var("BACKEND_URL")?));
    auth_config.session.cookie_same_site = SameSite::None;

    let auth = Arc::new(
        AuthBuilder::new(auth_config)
            .csrf(CsrfConfig::new().enabled(false))
            .database(hooked)
            .plugin(EmailPasswordPlugin::new())
            .plugin(PasswordManagementPlugin::new())
            .plugin(SessionManagementPlugin::new())
            .plugin(AccountManagementPlugin::new())
            .plugin(
                OAuthPlugin::new()
                    .add_provider(
                        "github",
                        OAuthProvider::github(&get_var("GITHUB_ID")?, &get_var("GITHUB_SECRET")?),
                    )
                    .add_provider(
                        "discord",
                        OAuthProvider::discord(
                            &get_var("DISCORD_ID")?,
                            &get_var("DISCORD_SECRET")?,
                        ),
                    )
                    .add_provider(
                        "google",
                        OAuthProvider::google(&get_var("GOOGLE_ID")?, &get_var("GOOGLE_SECRET")?),
                    )
                    .add_provider(
                        "hca",
                        better_auth_hca::oauth(get_var("HCA_ID")?, get_var("HCA_SECRET")?),
                    )
                    .add_provider(
                        "slack",
                        slack_oauth::oauth(get_var("SLACK_ID")?, get_var("SLACK_SECRET")?),
                    ),
            )
            .plugin(DeviceAuthorizationPlugin::with_config(
                DeviceAuthorizationConfig {
                    enabled: true,
                    verification_uri: format!("{}/device", get_var("FRONTEND_URL")?),
                    interval: 5,
                    expires_in: 1800,
                },
            ))
            .build()
            .await
            .map_err(|e| {
                make_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create auth config: {e}"),
                )
            })?,
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

    let app = Router::new()
        .nest("/auth", auth_router)
        .merge(api::router())
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
        let frontend_url = get_var("FRONTEND_URL").unwrap();
        let (mut parts, body) = response.into_parts();
        let token = axum::body::to_bytes(body, 1024 * 64)
            .await
            .ok()
            .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
            .and_then(|v| v["token"].as_str().map(String::from));

        let redirect_url = if let Some(t) = &token {
            format!("{}?token={}", frontend_url, t)
        } else {
            frontend_url
        };

        parts.status = StatusCode::FOUND;
        parts
            .headers
            .insert(header::LOCATION, redirect_url.parse().unwrap());
        parts.headers.remove(header::CONTENT_TYPE);

        Ok(Response::from_parts(parts, ().into()))
    } else {
        Ok(response)
    }
}

async fn cors_middleware(req: Request<body::Body>, next: Next) -> Result<Response<Body>, WIError> {
    let frontend_url = get_var("FRONTEND_URL")?;
    let path = req.uri().path().to_string();
    let is_public = !path.starts_with("/auth") && path != "/runs/new" && path != "/player";
    let request_origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let allowed_origin = if is_public {
        "*"
    } else {
        frontend_url.as_str()
    };
    let cors_config = CorsConfig::new();

    if req.method() == Method::OPTIONS {
        if !is_public && request_origin != frontend_url {
            return Ok(Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .expect("Failed to build response"));
        }

        let mut res = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origin)
            .header(
                ACCESS_CONTROL_ALLOW_METHODS,
                cors_config.allowed_methods.join(", "),
            )
            .header(
                ACCESS_CONTROL_ALLOW_HEADERS,
                cors_config.allowed_headers.join(", "),
            )
            .header(
                ACCESS_CONTROL_ALLOW_CREDENTIALS,
                cors_config.allow_credentials.to_string(),
            )
            .header(ACCESS_CONTROL_MAX_AGE, cors_config.max_age)
            .body(Body::empty())
            .unwrap();
        if !is_public {
            res.headers_mut().insert(
                ACCESS_CONTROL_ALLOW_CREDENTIALS,
                cors_config.allow_credentials.to_string().parse().unwrap(),
            );
        }
        return Ok(res);
    }
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, allowed_origin.parse().unwrap());
    if !is_public {
        headers.insert(
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            cors_config.allow_credentials.to_string().parse().unwrap(),
        );
    }
    Ok(response)
}
