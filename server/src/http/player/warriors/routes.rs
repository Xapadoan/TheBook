use axum::{
    routing::{get, patch, post}, Router
};

use crate::http::middlewares::get_player_warrior;

use super::{
    gen_random_warrior::gen_random_warrior,
    level_up::level_up,
    read::read_warrior,
    remove_from_replay::remove_warrior_from_replay,
    remove_warrior::remove_warrior,
    replace_protection::replace_protection,
    replace_weapon::replace_weapon,
};

pub fn player_warriors_routes() -> Router {
    let single_warrior_routes = Router::new()
        .route("/", get(read_warrior).delete(remove_warrior))
        .route("/level-up", patch(level_up))
        .route("/replace-weapon", patch(replace_weapon))
        .route("/replace-protection", patch(replace_protection))
        .route("/remove-from-replay", patch(remove_warrior_from_replay))
        .layer(axum::middleware::from_fn(get_player_warrior));
    Router::new()
        .route("/random", post(gen_random_warrior))
        .nest("/:warrior_id", single_warrior_routes)
}