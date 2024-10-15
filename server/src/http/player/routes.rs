use axum::{routing::get, Router};

use super::{auth::player_auth, read::read_player, shop::player_shop_routes, tournaments::player_tournaments_routes, warriors::player_warriors_routes};

pub fn player_routes() -> Router {
    Router::new()
        .route("/", get(read_player))
        .nest("/shop", player_shop_routes())
        .nest("/tournaments", player_tournaments_routes())
        .nest("/warriors", player_warriors_routes())
        .layer(axum::middleware::from_fn(player_auth))
}
