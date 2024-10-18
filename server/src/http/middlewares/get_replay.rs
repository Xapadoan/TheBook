use axum::{
    extract::{Path, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::replay::ReplayManager;

pub async fn get_replay(
    Path(tournament_uuid): Path<Uuid>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let manager = ReplayManager::new(&tournament_uuid);
    let replay = manager.get_tournament_replay();
    if replay.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    req.extensions_mut().insert(replay.unwrap());
    Ok(next.run(req).await)
}