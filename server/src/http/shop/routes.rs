use std::sync::Arc;

use axum::{routing::get, Router};

use crate::http::app::AppState;

use super::read::read_shop;

pub fn shop_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(read_shop))
}
