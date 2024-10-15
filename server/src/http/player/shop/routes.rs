use axum::{routing::patch, Router};

use super::{buy::buy_item, sell::sell_item};

pub fn player_shop_routes() -> Router {
    Router::new()
        .route("/sell/:slot_uuid", patch(sell_item))
        .route("/buy/:slot_uuid", patch(buy_item))
}