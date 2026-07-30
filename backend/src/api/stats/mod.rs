use serde::Deserialize;

pub mod artifacts;
pub mod combined;
pub mod difficulties;
pub mod overall;
pub mod stages;
pub mod survivors;

#[derive(Deserialize, Default)]
pub struct UsernameQuery {
    username: Option<String>,
}
