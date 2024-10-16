use axum::{
    routing::{get, patch},
    Router,
};

use crate::http::middlewares::get_player_warrior;

use super::{
    replace_weapon::replace_weapon,
    level_up::level_up,
    read::read_warrior,
    remove_from_replay::remove_warrior_from_replay
};

pub fn player_warriors_routes() -> Router {
    let single_warrior_routes = Router::new()
        .route("/", get(read_warrior))
        .route("/level-up", patch(level_up))
        .route("/replace-weapon", patch(replace_weapon))
        .route("/remove-from-replay", patch(remove_warrior_from_replay))
        .layer(axum::middleware::from_fn(get_player_warrior));
    Router::new()
        .nest("/:warrior_id", single_warrior_routes)
}