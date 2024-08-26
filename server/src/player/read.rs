use shared::health::MutablePassiveHealing;
use shared::player::Player;
use shared::warrior::MutableWarriorCollection;
use uuid::Uuid;

use crate::auth::SessionManager;

use super::PlayerAPIError;

pub fn read_player(session_uuid: &Uuid) -> Result<Player, PlayerAPIError> {
    let manager = SessionManager::build()?;
    let mut player = manager.read_player(session_uuid)?;
    for warrior in player.warriors_mut() {
        warrior.passive_heal();
    }
    Ok(player)
}
