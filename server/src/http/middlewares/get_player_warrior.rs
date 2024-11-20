use axum::{extract::{Path, Request}, http::StatusCode, middleware::Next, response::Response, Extension};
use shared::{player::Player, unique_entity::UniqueEntity};
use uuid::Uuid;

use crate::player::PlayerManager;

pub async fn get_player_warrior(
    Extension(player): Extension<Player>,
    Path(warrior_uuid): Path<Uuid>,
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {
    eprintln!(
        "[DEBUG] reading warrior {} for player {}",
        warrior_uuid,
        player.uuid().to_string(),
    );
    let manager = PlayerManager::new(&player);
    match manager.read_warrior(&warrior_uuid) {
        None => Err(StatusCode::NOT_FOUND),
        Some(warrior) => {
            req.extensions_mut().insert(warrior.clone());
            Ok(next.run(req).await)
        }
    }
}