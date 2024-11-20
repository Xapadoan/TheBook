use std::sync::Arc;

use axum::{routing::{get, patch}, Router};

use crate::http::{app::AppState, middlewares::session_auth};

use super::{
    read::read_player,
    tournaments::player_tournaments_routes,
    warriors::player_warriors_routes,
    buy_item::buy_item,
    sell_item::sell_item,
};

pub fn player_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(read_player))
        .route("/buy-item/:slot_uuid", patch(buy_item))
        .route("/sell-item/:slot_uuid", patch(sell_item))
        .nest("/tournaments", player_tournaments_routes(app_state.clone()))
        .nest("/warriors", player_warriors_routes())
        .layer(axum::middleware::from_fn_with_state(app_state, session_auth))
}
