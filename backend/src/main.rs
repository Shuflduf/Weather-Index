use std::{collections::HashMap, env, error::Error, fs::File, io::Read, sync::Arc};

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
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    auth_entities::AppAdapter,
    entity::{account, run_report, session, user, verification},
    error::WIError,
    run_report_dto::RunReportDTO,
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
        .base_url("http://localhost:3000/auth");
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
        .nest("/auth", auth_router)
        .route("/auth/callback/github", get(github_callback))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/new-run", post(insert_new_run))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn random_hex(len: usize) -> String {
    let mut buf = vec![0u8; len];
    File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut buf))
        .expect("failed to read /dev/urandom");
    buf.iter().map(|b| format!("{:02x}", b)).collect()
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
                Json(serde_json::json!({"error": format!("DB error: {e}")})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid state"})),
            )
        })?;

    let state_value: serde_json::Value =
        serde_json::from_str(&verification.value).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to parse state: {e}")})),
            )
        })?;

    let code_verifier = state_value["code_verifier"]
        .as_str()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Missing code_verifier in state"})),
            )
        })?
        .to_string();
    let callback_url = state_value["callback_url"]
        .as_str()
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Missing callback_url in state"})),
            )
        })?
        .to_string();

    verification::Entity::delete_by_id(verification.id)
        .exec(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to delete state: {e}")})),
            )
        })?;

    let client = reqwest::Client::new();
    let token_resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", env::var("GITHUB_ID").unwrap().as_str()),
            ("client_secret", env::var("GITHUB_SECRET").unwrap().as_str()),
            ("code", code.as_str()),
            ("code_verifier", &code_verifier),
            ("redirect_uri", &callback_url),
        ])
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Token exchange failed: {e}")})),
            )
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Bad token response: {e}")})),
            )
        })?;

    let access_token = token_resp["access_token"]
        .as_str()
        .ok_or_else(|| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": "No access_token in GitHub response", "body": token_resp})),
            )
        })?
        .to_string();

    let user_resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "rordb")
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("GitHub user API failed: {e}")})),
            )
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Bad user response: {e}")})),
            )
        })?;

    let github_id = user_resp["id"].as_i64().ok_or_else(|| {
        (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": "No id in GitHub user response"})),
        )
    })?;
    let github_login = user_resp["login"].as_str().unwrap_or("unknown");
    let user_name = user_resp["name"].as_str();
    let avatar_url = user_resp["avatar_url"].as_str();

    let emails = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "rordb")
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("GitHub emails API failed: {e}")})),
            )
        })?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Bad emails response: {e}")})),
            )
        })?;

    let primary_email = emails
        .iter()
        .find(|e| e["primary"].as_bool() == Some(true))
        .or_else(|| emails.first())
        .and_then(|e| e["email"].as_str());

    let existing_account = account::Entity::find()
        .filter(account::Column::ProviderId.eq("github"))
        .filter(account::Column::AccountId.eq(github_id.to_string()))
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("DB error: {e}")})),
            )
        })?;

    let user_id = if let Some(acct) = existing_account {
        acct.user_id
    } else {
        let existing_user = if let Some(email) = primary_email {
            user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(&state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": format!("DB error: {e}")})),
                    )
                })?
        } else {
            None
        };

        let uid = if let Some(u) = existing_user {
            u.id
        } else {
            let uid = random_hex(16);
            let now = chrono::Utc::now();
            user::ActiveModel {
                id: Set(uid.clone()),
                name: Set(user_name.map(|s| s.to_string())),
                email: Set(primary_email.map(|s| s.to_string())),
                email_verified: Set(true),
                image: Set(avatar_url.map(|s| s.to_string())),
                username: Set(Some(github_login.to_string())),
                display_username: Set(None),
                two_factor_enabled: Set(false),
                role: Set(None),
                banned: Set(false),
                ban_reason: Set(None),
                ban_expires: Set(None),
                metadata: Set(serde_json::json!({})),
                created_at: Set(now),
                updated_at: Set(now),
            }
            .insert(&state.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": format!("Failed to create user: {e}")})),
                )
            })?;
            uid
        };

        account::ActiveModel {
            id: Set(random_hex(16)),
            account_id: Set(github_id.to_string()),
            provider_id: Set("github".to_string()),
            user_id: Set(uid.clone()),
            access_token: Set(Some(access_token.clone())),
            refresh_token: Set(None),
            id_token: Set(None),
            access_token_expires_at: Set(None),
            refresh_token_expires_at: Set(None),
            scope: Set(None),
            password: Set(None),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        }
        .insert(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to create account: {e}")})),
            )
        })?;

        uid
    };

    let session_token = random_hex(32);
    let now = chrono::Utc::now();
    let expires = now + chrono::Duration::days(7);
    session::ActiveModel {
        id: Set(random_hex(16)),
        expires_at: Set(expires),
        token: Set(session_token.clone()),
        created_at: Set(now),
        updated_at: Set(now),
        ip_address: Set(None),
        user_agent: Set(None),
        user_id: Set(user_id),
        impersonated_by: Set(None),
        active_organization_id: Set(None),
        active: Set(true),
    }
    .insert(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to create session: {e}")})),
        )
    })?;

    let cookie = format!(
        "better-auth.session-token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
        session_token
    );
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
