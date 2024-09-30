use shared::player::Player;
use uuid::Uuid;

use crate::auth::SessionManager;

use super::PlayerAPIError;

pub fn read_player(session_uuid: &Uuid) -> Result<Player, PlayerAPIError> {
    let manager = SessionManager::build()?;
    let player = manager.read_player(session_uuid)?;
    Ok(player)
}
