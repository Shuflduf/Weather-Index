use std::{collections::HashMap, env, error::Error, sync::Arc};

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use better_auth::{
    plugins::{
        oauth::OAuthProvider, AccountManagementPlugin, EmailPasswordPlugin, OAuthPlugin,
        PasswordManagementPlugin, SessionManagementPlugin,
    },
    AuthBuilder, AuthConfig, AxumIntegration, BetterAuth, CsrfConfig,
};
use better_auth_core::utils::cookie_utils::create_session_cookie;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    auth_entities::AppAdapter, entity::{account, run_report, user, verification},
    error::WIError, run_report_dto::RunReportDTO,
};

mod auth_entities;
mod db;
mod entity;
mod error;
mod run_report_dto;

struct WIState {
    db: DatabaseConnection,
    auth: Arc<BetterAuth<AppAdapter>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "rordb_backend=debug,better_auth=debug,better_auth_core=debug,warn",
        ))
        .init();
    dotenvy::dotenv().ok();

    let db = db::init_db().await.expect("Failed to initialize database");
    let pg_pool = db.get_postgres_connection_pool();

    for table in &["sessions", "users", "accounts", "verifications"] {
        sqlx::query(&format!(
            "ALTER TABLE {table} ALTER COLUMN updated_at SET DEFAULT NOW()"
        ))
        .execute(pg_pool)
        .await?;
    }

    let adapter = AppAdapter::from_pool(pg_pool.clone());

    let auth_config = AuthConfig::new(env::var("ENCRYPTION_KEY")?)
        .trusted_origins(vec!["http://localhost:5173".into()])
        .base_url("http://localhost:5173/auth");
    let auth = Arc::new(
        AuthBuilder::new(auth_config)
            .csrf(CsrfConfig::new().enabled(false))
            .database(adapter)
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
        .route("/auth/callback/github", get(github_callback))
        .nest("/auth", auth_router)
        .route("/", get(|| async { "Hello, World!" }))
        .route("/new-run", post(insert_new_run))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn github_callback(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<WIState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let code = params.get("code").ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing code parameter"})),
        )
    })?;
    let state_param = params.get("state").ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing state parameter"})),
        )
    })?;

    let verification = verification::Entity::find()
        .filter(verification::Column::Identifier.eq(format!("oauth:{}", state_param)))
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("DB lookup failed: {e}")})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid or expired OAuth state"})),
            )
        })?;

    let payload: serde_json::Value =
        serde_json::from_str(&verification.value).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Invalid state payload: {e}")})),
            )
        })?;

    let code_verifier = payload["code_verifier"].as_str().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Missing code_verifier in state"})),
        )
    })?;
    let callback_url = payload["callback_url"].as_str().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Missing callback_url in state"})),
        )
    })?;

    verification::Entity::delete_by_id(verification.id)
        .exec(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to delete verification: {e}")})),
            )
        })?;

    let client = reqwest::Client::builder()
        .user_agent("rordb-backend/0.1.0")
        .build()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to build HTTP client: {e}")})),
            )
        })?;

    let github_id =
        env::var("GITHUB_ID").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Missing GITHUB_ID"})),
            )
        })?;
    let github_secret =
        env::var("GITHUB_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Missing GITHUB_SECRET"})),
            )
        })?;

    let token_resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", callback_url),
            ("client_id", &github_id),
            ("client_secret", &github_secret),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Token exchange failed: {e}")})),
            )
        })?;

    if !token_resp.status().is_success() {
        let body = token_resp.text().await.unwrap_or_default();
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Token exchange error: {body}")})),
        ));
    }

    let token_data: serde_json::Value = token_resp.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to parse token response: {e}")})),
        )
    })?;

    let access_token = token_data["access_token"].as_str().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Missing access_token in token response"})),
        )
    })?;

    let user_resp = client
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("User info request failed: {e}")})),
            )
        })?;

    if !user_resp.status().is_success() {
        let body = user_resp.text().await.unwrap_or_default();
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("User info error: {body}")})),
        ));
    }

    let user_data: serde_json::Value = user_resp.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to parse user info: {e}")})),
        )
    })?;

    let gh_user_id = user_data["id"]
        .as_i64()
        .map(|i| i.to_string())
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Missing GitHub user id"})),
            )
        })?;
    let email = user_data["email"].as_str().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "No public email on GitHub account"})),
        )
    })?;
    let name = user_data["name"].as_str().unwrap_or(email);
    let avatar = user_data["avatar_url"].as_str().map(String::from);

    let now = chrono::Utc::now();

    let user_id = {
        let existing_account = account::Entity::find()
            .filter(account::Column::ProviderId.eq("github"))
            .filter(account::Column::AccountId.eq(&gh_user_id))
            .one(&state.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Account lookup failed: {e}")})),
                )
            })?;

        if let Some(acct) = existing_account {
            acct.user_id
        } else {
            let existing_user = user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(&state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": format!("User lookup failed: {e}")})),
                    )
                })?;

            match existing_user {
                Some(u) => u.id,
                None => {
                    let new_id = uuid::Uuid::new_v4().to_string();

                    user::ActiveModel {
                        id: Set(new_id.clone()),
                        email: Set(Some(email.to_string())),
                        name: Set(Some(name.to_string())),
                        image: Set(avatar),
                        email_verified: Set(true),
                        two_factor_enabled: Set(false),
                        banned: Set(false),
                        metadata: Set(serde_json::Value::Object(Default::default())),
                        created_at: Set(now),
                        updated_at: Set(now),
                        ..Default::default()
                    }
                    .insert(&state.db)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({"error": format!("User insert failed: {e}")})),
                        )
                    })?;

                    new_id
                }
            }
        }
    };

    let already_linked = account::Entity::find()
        .filter(account::Column::UserId.eq(&user_id))
        .filter(account::Column::ProviderId.eq("github"))
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Account link check failed: {e}")})),
            )
        })?
        .is_some();

    if !already_linked {
        account::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            account_id: Set(gh_user_id),
            provider_id: Set("github".to_string()),
            user_id: Set(user_id.clone()),
            access_token: Set(Some(access_token.to_string())),
            refresh_token: Set(None),
            id_token: Set(None),
            access_token_expires_at: Set(None),
            refresh_token_expires_at: Set(None),
            scope: Set(Some("user:email".to_string())),
            password: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Account insert failed: {e}")})),
            )
        })?;
    }

    let user_model = user::Entity::find_by_id(&user_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("User fetch failed: {e}")})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Created user not found"})),
            )
        })?;

    let session = state
        .auth
        .session_manager()
        .create_session(&user_model, None, None)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Session creation failed: {e}")})),
            )
        })?;

    let cookie = create_session_cookie(&session.token, state.auth.config());

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("http://localhost:5173/")))
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
