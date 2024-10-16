use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};
use uuid::Uuid;

use crate::player::read_player;

pub async fn session_auth(
    headers: HeaderMap,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    match get_session_uuid(&headers) {
        Some(uuid) => {
            let player = read_player(&uuid);
            if player.is_ok() {
                req.extensions_mut().insert(player.unwrap());
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::NOT_FOUND)
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
