use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    auth::SessionManager,
    http::app::AppState,
    repository::{sql_repository::players::PlayersRepository, RepositoryRead},
};

pub async fn session_auth(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    match get_session_uuid(&headers) {
        Some(uuid) => {
            eprintln!("[DEBUG] Reading session {}", uuid.to_string());
            let session_manager = SessionManager::build();
            if session_manager.is_err() {
                eprintln!("[ERROR] Failed to create session manager");
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            let session_manager = session_manager.unwrap();
            let session = session_manager.read_session(&uuid).await;
            if let Err(err) = session {
                eprintln!("[ERROR] Session Error: {err}");
                return Err(StatusCode::NOT_FOUND);
            }
            let session = session.unwrap();
            let player_repository = PlayersRepository::new(state.db_pool());
            let player = player_repository.read(session.player_uuid()).await;
            match player {
                Ok(player) => {
                    req.extensions_mut().insert(player);
                    Ok(next.run(req).await)
                },
                Err(err) => {
                    eprintln!("[ERROR] Session Error:\n{err}");
                    Err(StatusCode::NOT_FOUND)
                },
            }
        },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_session_uuid(headers: &HeaderMap) -> Option<Uuid> {
    match headers.get("x-session-id") {
        Some(value) => {
            let str = value.to_str();
            if str.is_ok() {
                let session_uuid = Uuid::parse_str(str.unwrap());
                if session_uuid.is_ok() {
                    return Some(session_uuid.unwrap());
                }
            }
            return None;
        },
        None => None,
    }
}
