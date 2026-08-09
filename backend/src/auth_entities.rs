use crate::entity::{session, user};

pub type AppUser = user::Model;
pub type AppSession = session::Model;

pub type AppAdapter = better_auth::adapters::SqlxAdapter<AppUser, AppSession>;
pub type AppHookedAdapter = better_auth::HookedDatabaseAdapter<AppAdapter>;
