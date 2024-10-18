use shared::{player::Player, unique_entity::UniqueEntity, warrior::{Warrior, WarriorCollection}};
use uuid::Uuid;

pub struct PlayerManager<'a> {
    player: &'a Player
}

impl<'a> PlayerManager<'a> {
    pub fn new(player: &'a Player) -> Self {
        Self { player }
    }
    pub fn read_warrior(&self, warrior_uuid: &Uuid) -> Option<&Warrior> {
        self.player.warriors().iter().find(
            |w| w.uuid() == warrior_uuid
        )
    }
}
