use axum::{routing::{get, patch}, Router};

use crate::http::middlewares::session_auth;

use super::{
    read::read_player,
    tournaments::player_tournaments_routes,
    warriors::player_warriors_routes,
    buy_item::buy_item,
    sell_item::sell_item,
};

pub fn player_routes() -> Router {
    Router::new()
        .route("/", get(read_player))
        .route("/buy-item/:slot_uuid", patch(buy_item))
        .route("/sell-item/:slot_uuid", patch(sell_item))
        .nest("/tournaments", player_tournaments_routes())
        .nest("/warriors", player_warriors_routes())
        .layer(axum::middleware::from_fn(session_auth))
}
