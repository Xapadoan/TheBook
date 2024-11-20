use std::sync::Arc;

use axum::{routing::{get, patch}, Router};

use crate::http::{app::AppState, middlewares::get_tournament};

use super::{new_replays::new_replays, register_warriors::register_warriors};

pub fn player_tournaments_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let single_tournament_router = Router::new()
        .route("/register", patch(register_warriors))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), get_tournament));
    Router::new()
        .nest("/:tournament_uuid", single_tournament_router)
        .route("/new-replays", get(new_replays))
}
