use server::repository::{PlayerDTOFile, PlayerRepository, Repository, RepositoryError};
use shared::{inventory::Inventory, player::Player, warrior::Warrior};
use uuid::Uuid;

mod player_repository;

pub fn create_player<T: Repository<PlayerDTOFile>, K: Repository<Warrior>>(
    repo: &PlayerRepository<T, K>,
    warriors: Vec<Warrior>,
) -> Result<Player, RepositoryError> {
    let player = Player::new(
        Uuid::new_v4(),
        "test".to_string(),
        "Test".to_string(),
        warriors,
        Inventory::new(),
    );
    repo.create(&player)?;
    Ok(player)
}