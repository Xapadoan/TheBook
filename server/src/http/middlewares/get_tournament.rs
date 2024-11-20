use std::sync::Arc;

use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{http::app::AppState, repository::sql_repository::tournaments::TournamentsRepository, tournament::manager::TournamentManager};

pub async fn get_tournament(
    Path(tournament_uuid): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let repo = TournamentsRepository::new(state.db_pool());
    let manager = TournamentManager::new(repo);
    match manager.get_tournament(&tournament_uuid).await {
        Err(_) => Err(StatusCode::NOT_FOUND),
        Ok(tournament) => {
            req.extensions_mut().insert(tournament);
            Ok(next.run(req).await)
        }
    }
}
