use axum::Router;

use super::{player_warrior::player_warrior, tournaments::player_warrior_tournaments_routes};

pub fn player_warriors_routes() -> Router {
    let single_warrior_routes = Router::new()
        .nest("/tournaments", player_warrior_tournaments_routes())
        .layer(axum::middleware::from_fn(player_warrior));
    Router::new()
        .nest("/:warrior_id", single_warrior_routes)
}