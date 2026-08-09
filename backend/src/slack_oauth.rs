use better_auth::plugins::oauth::{OAuthProvider, OAuthUserInfo};

pub fn oauth(client_id: impl Into<String>, client_secret: impl Into<String>) -> OAuthProvider {
    OAuthProvider {
        client_id: client_id.into(),
        client_secret: client_secret.into(),
        auth_url: "https://slack.com/openid/connect/authorize".into(),
        token_url: "https://slack.com/api/openid.connect.token".into(),
        user_info_url: "https://slack.com/api/openid.connect.userInfo".into(),
        scopes: vec!["openid", "profile", "email"]
            .into_iter()
            .map(String::from)
            .collect(),
        map_user_info: |v| {
            if v["ok"].as_bool() == Some(false) {
                return Err(format!(
                    "Slack userinfo error: {}",
                    v["error"].as_str().unwrap_or("Unknown")
                ));
            }
            Ok(OAuthUserInfo {
                id: v["sub"].as_str().ok_or("Missing sub")?.to_string(),
                email: v["email"].as_str().ok_or("Missing email")?.to_string(),
                name: v["name"].as_str().map(String::from),
                image: v["picture"].as_str().map(String::from),
                email_verified: v["email_verified"].as_bool().unwrap_or(false),
            })
        },
    }
}
