use std::sync::Arc;

use async_trait::async_trait;
use better_auth::{AuthResult, CreateUser, DatabaseHooks, UserOps};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::auth_entities::AppAdapter;

const WORDS: &[&str] = &["amber", "azure", "breezy"];

fn random_username() -> String {
    let mut rng = thread_rng();
    format!(
        "{}-{}-{}",
        WORDS.choose(&mut rng).unwrap(),
        WORDS.choose(&mut rng).unwrap(),
        WORDS.choose(&mut rng).unwrap()
    )
}

pub struct RandomUsernameHook {
    adapter: Arc<AppAdapter>,
}

impl RandomUsernameHook {
    pub fn new(adapter: Arc<AppAdapter>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl DatabaseHooks<AppAdapter> for RandomUsernameHook {
    async fn before_create_user(&self, user: &mut CreateUser) -> AuthResult<()> {
        if user.username.is_none() {
            loop {
                let username = random_username();
                if self
                    .adapter
                    .get_user_by_username(&username)
                    .await?
                    .is_none()
                {
                    user.username = Some(username.clone());
                    break;
                }
            }
        }
        Ok(())
    }
}
