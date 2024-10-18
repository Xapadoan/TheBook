use axum::{routing::get, Router};

use super::playable::playable_tournament;

pub fn tournaments_routes() -> Router {
    Router::new()
        .route("/playable", get(playable_tournament))
}
