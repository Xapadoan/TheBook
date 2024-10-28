use std::sync::Arc;

use axum::{routing::get, Router};

use crate::http::app::AppState;

use super::playable::playable_tournament;

pub fn tournaments_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/playable", get(playable_tournament))
}
