use std::{collections::HashMap, env, sync::Arc};

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use rand::RngCore;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;

use crate::{
    entity::{account, session, user, verification},
    error::{db_error, make_error},
    WIState,
};
pub async fn handle_callback(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<WIState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let code = params
        .get("code")
        .ok_or_else(|| make_error(StatusCode::BAD_REQUEST, "Missing code parameter".into()))?;
    let state_param = params
        .get("state")
        .ok_or_else(|| make_error(StatusCode::BAD_REQUEST, "Missing state parameter".into()))?;
    println!("{code}, {state_param}");

    // return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))));

    let verification = verification::Entity::find()
        .filter(verification::Column::Identifier.eq(format!("oauth:{}", state_param)))
        .one(&state.db)
        .await
        .map_err(db_error)?
        .ok_or_else(|| make_error(StatusCode::BAD_REQUEST, "Invalid state".into()))?;

    let state_value: serde_json::Value =
        serde_json::from_str(&verification.value).map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse state: {e}"),
            )
        })?;

    let code_verifier = state_value["code_verifier"]
        .as_str()
        .ok_or_else(|| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing code_verifier in state".into(),
            )
        })?
        .to_string();
    let callback_url = state_value["callback_url"]
        .as_str()
        .ok_or_else(|| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing callback_url in state".into(),
            )
        })?
        .to_string();

    verification::Entity::delete_by_id(verification.id)
        .exec(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete state: {e}"),
            )
        })?;

    let client = reqwest::Client::new();
    let token_resp = client
        .post("https://discord.com/api/oauth2/token")
        .header("Accept", "application/json")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", env::var("DISCORD_ID").unwrap().as_str()),
            (
                "client_secret",
                env::var("DISCORD_SECRET").unwrap().as_str(),
            ),
            ("code", code.as_str()),
            ("code_verifier", &code_verifier),
            ("redirect_uri", &callback_url),
        ])
        .send()
        .await
        .map_err(|e| {
            make_error(
                StatusCode::BAD_GATEWAY,
                format!("Token exchange failed: {e}"),
            )
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| make_error(StatusCode::BAD_GATEWAY, format!("Bad token response: {e}")))?;

    let access_token = token_resp["access_token"]
        .as_str()
        .ok_or_else(|| {
            make_error(
                StatusCode::BAD_GATEWAY,
                "No access_token in Discord response".into(),
            )
        })?
        .to_string();

    let user_resp = client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await
        .map_err(|e| {
            make_error(
                StatusCode::BAD_GATEWAY,
                format!("Discord user API failed: {e}"),
            )
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| make_error(StatusCode::BAD_GATEWAY, format!("Bad user response: {e}")))?;

    println!("{user_resp:#?}");
    let discord_id = user_resp["id"]
        .as_str()
        .ok_or_else(|| {
            make_error(
                StatusCode::BAD_GATEWAY,
                "No id in GitHub user response".into(),
            )
        })?
        .to_string();

    let discord_username = user_resp["username"].as_str().unwrap_or("Unknown");
    let display_name = user_resp["global_name"].as_str();
    let avatar_url = user_resp["avatar"]
        .as_str()
        .map(|hash| format!("https://cdn.discordapp.com/avatars/{discord_id}/{hash}"));
    let primary_email = user_resp["email"].as_str();
    let email_verified = user_resp["verified"].as_bool().unwrap_or(false);

    let existing_account = account::Entity::find()
        .filter(account::Column::ProviderId.eq("discord"))
        .filter(account::Column::AccountId.eq(discord_id.to_string()))
        .one(&state.db)
        .await
        .map_err(db_error)?;

    let user_id = if let Some(acct) = existing_account {
        acct.user_id
    } else {
        let existing_user = if let Some(email) = primary_email {
            user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(&state.db)
                .await
                .map_err(db_error)?
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
                name: Set(Some(discord_username.to_string())),
                email: Set(primary_email.map(|s| s.to_string())),
                email_verified: Set(email_verified),
                image: Set(avatar_url.map(|s| s.to_string())),
                username: Set(Some(discord_username.to_string())),
                display_username: Set(None),
                two_factor_enabled: Set(false),
                role: Set(None),
                banned: Set(false),
                ban_reason: Set(None),
                ban_expires: Set(None),
                metadata: Set(json!({})),
                created_at: Set(now),
                updated_at: Set(now),
                about_me: Set(None),
                region: Set(None),
            }
            .insert(&state.db)
            .await
            .map_err(|e| {
                make_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create user: {e}"),
                )
            })?;
            uid
        };

        account::ActiveModel {
            id: Set(random_hex(16)),
            account_id: Set(discord_id.to_string()),
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
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create account: {e}"),
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
        make_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create session: {e}"),
        )
    })?;

    let cookie = format!(
        "better-auth.session-token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800",
        session_token
    );
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());
    Ok((
        headers,
        Redirect::to(&env::var("FRONTEND_URL").map_err(|_| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "FRONTEND_URL variable doesnt exist".into(),
            )
        })?),
    ))
}

fn random_hex(len: usize) -> String {
    let mut buf = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut buf);
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}
