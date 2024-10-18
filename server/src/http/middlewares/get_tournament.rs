use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::tournament::manager::TournamentManager;

pub async fn get_tournament(
    Path(tournament_uuid): Path<Uuid>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let manager = TournamentManager::build();
    if manager.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let manager = manager.unwrap();
    match manager.get_tournament(&tournament_uuid) {
        Err(_) => Err(StatusCode::NOT_FOUND),
        Ok(tournament) => {
            req.extensions_mut().insert(tournament);
            Ok(next.run(req).await)
        }
    }
}
