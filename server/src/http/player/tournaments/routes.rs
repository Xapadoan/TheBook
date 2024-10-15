use axum::{routing::get, Router};

use super::new_replays::new_replays;

pub fn player_tournaments_routes() -> Router {
    Router::new()
        .route("/new-replays", get(new_replays))
}
