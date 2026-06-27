use crate::entity::{account, session, user, verification};

pub type AppUser = user::Model;
pub type AppSession = session::Model;
pub type AppAccount = account::Model;
pub type AppVerification = verification::Model;

pub type AppAdapter =
    better_auth::adapters::SqlxAdapter<AppUser, AppSession, AppAccount, AppVerification>;
