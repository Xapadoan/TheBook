use shared::{
    experience::{ExperienceError, GainExperience},
    stats::StatKind,
    unique_entity::UniqueEntity, warrior::MutableWarriorCollection,
};
use uuid::Uuid;

use crate::{player::PlayerAPIError, repository::{PlayerRepository, Repository}};

pub fn level_up(player_uuid: &Uuid, warrior_uuid: &Uuid, stat: &StatKind) -> Result<(), PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(&player_uuid)?;
    let warrior = player.warriors_mut().iter_mut().find(
        |warrior| { warrior.uuid() == warrior_uuid }
    );
    if let None = warrior {
        eprintln!(
            "[WARN] Warrior {} not found for player {}",
            warrior_uuid,
            player_uuid,
        )
    }
    let warrior = warrior.unwrap();
    warrior.level_up(stat)?;
    player_repo.update(player.uuid(), &player)?;
    Ok(())
}

impl From<ExperienceError> for PlayerAPIError {
    fn from(value: ExperienceError) -> Self {
        Self::new(&format!("Experience Error:\n{value}"))
    }
}
