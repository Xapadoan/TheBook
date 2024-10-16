use axum::{routing::patch, Router};

use crate::http::middlewares::get_player_warrior;

use super::remove_from_replay::remove_warrior_from_replay;

pub fn player_warriors_routes() -> Router {
    let single_warrior_routes = Router::new()
        .route("/remove-from-replay", patch(remove_warrior_from_replay))
        .layer(axum::middleware::from_fn(get_player_warrior));
    Router::new()
        .nest("/:warrior_id", single_warrior_routes)
}