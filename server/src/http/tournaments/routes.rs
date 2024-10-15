use axum::{routing::get, Router};

use super::replay::read_tournament_replay;

pub fn tournaments_routes() -> Router {
    Router::new()
        .route("/:tournament/replay", get(read_tournament_replay))
}
