use std::sync::Arc;

use axum::{routing::get, Router};

use crate::WIState;

mod get_run_reports;

pub fn router() -> Router<Arc<WIState>> {
    Router::new().route("/get-run-reports", get(get_run_reports::get))
}
