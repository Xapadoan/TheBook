use shared::inventory::Inventory;
use shared::name::Name;
use shared::player::{Player, PlayerBuildError, PlayerBuilder};
use shared::random::Random;
use shared::tournament::Tournament;
use shared::warrior::Warrior;
use uuid::Uuid;

pub struct BotPlayerBuilder<'a> {
    tournament: &'a Tournament,
    warriors: Vec<Warrior>,
}

impl<'a> BotPlayerBuilder<'a> {
    pub fn new(tournament: &'a Tournament) -> Self {
        Self {
            tournament: tournament,
            warriors: vec![],
        }
    }
}

impl<'a> PlayerBuilder for BotPlayerBuilder<'a> {
    fn build_username(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_display_name(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_warriors(&mut self) -> Result<(), PlayerBuildError> {
        let warriors_missing = self.tournament.max_contestants() - self.tournament.number_of_contestants();
        let mut i = 0;
        while i < warriors_missing {
            let warrior = Warrior::random();
            self.warriors.push(warrior);
            i += 1;
        }
        Ok(())
    }
    fn build_inventory(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build(self) -> Player {
        Player::new(
            Uuid::new_v4(),
            "bot".to_string(),
            format!("{} organizers", self.tournament.name()),
            self.warriors,
            Inventory::new(),
        )
    }
}