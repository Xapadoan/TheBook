use axum::{routing::get, Router};

use super::read::read_shop;

pub fn shop_routes() -> Router {
    Router::new().route("/", get(read_shop))
}
