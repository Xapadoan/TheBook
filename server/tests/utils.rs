use shared::{inventory::Inventory, player::Player, random::Random, warrior::{Warrior, WarriorCollection}};
use uuid::Uuid;

pub fn player_is_valid(player: &Player) -> bool {
    assert_eq!(
        player.warriors().len(),
        8,
        "Player has {} warriors instead of {}",
        player.warriors().len(),
        8,
    );

    true
}

pub fn test_player() -> Player {
    let mut warriors = vec![];
    let mut i = 0;
    while i < 8 {
        warriors.push(Warrior::random());
        i += 1;
    }

    let player = Player::new(
        Uuid::new_v4(),
        "test.1234".to_string(),
        "Test".to_string(),
        warriors,
        Inventory::new(),
    );
    assert!(player_is_valid(&player));

    player
}