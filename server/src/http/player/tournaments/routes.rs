use axum::{routing::{get, patch}, Router};

use crate::http::middlewares::get_tournament;

use super::{new_replays::new_replays, register_warriors::register_warriors};

pub fn player_tournaments_routes() -> Router {
    let single_tournament_router = Router::new()
        .route("/register", patch(register_warriors))
        .layer(axum::middleware::from_fn(get_tournament));
    Router::new()
        .nest("/:tournament_uuid", single_tournament_router)
        .route("/new-replays", get(new_replays))
}
