use axum::{middleware, routing::get, Router};

use crate::http::middlewares::get_replay;

use super::read::read_replay;

pub fn replay_routes() -> Router {
    let single_replay_router = Router::new()
        .route("/", get(read_replay))
        .layer(middleware::from_fn(get_replay));
    Router::new()
        .nest("/:tournament_uuid", single_replay_router)
}