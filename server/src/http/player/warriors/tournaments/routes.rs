use axum::{routing::patch, Router};

use super::remove_from_replay::remove_warrior_from_replay;

pub fn player_warrior_tournaments_routes() -> Router {
    Router::new()
        .route("/remove-from-replay", patch(remove_warrior_from_replay))
}
