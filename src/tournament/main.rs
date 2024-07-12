use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::PathBuf;

use crate::gen_random::GenRandom;
use crate::name::{HasName, Name};
use crate::random_dictionary::RandomDictionary;
use crate::repository::file_repository::FileRepository;
use crate::repository::main::{Repository, RepositoryError, UniqueEntity};
use crate::tournament::fight::FightResultKind;
use crate::warrior::Warrior;

use super::fight::Fight;
use super::name::TournamentNameDictionary;

#[derive(Debug)]
pub struct TournamentError {
    message: String,
}

impl TournamentError {
    fn new(message: String) -> Self {
        Self { message: format!("Tournament Error:\n{message}") }
    }
}

impl Display for TournamentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TournamentError {}

impl From<RepositoryError> for TournamentError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Tournament Repository Error:\n{value}"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    uuid: Uuid,
    name: Name,
    max_contestants: usize,
    contestants_ids: Vec<Uuid>,
}

impl Tournament {
    fn new(name: Name, max_contestants: usize) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            max_contestants,
            contestants_ids: vec![],
        }
    }

    pub fn add_contestant(&mut self, warrior: &Warrior) -> Result<(), TournamentError> {
        if self.contestants_ids.len() + 1 > self.max_contestants {
            return Err(TournamentError::new(String::from("Tournament will not allow more contestants")));
        }
        self.contestants_ids.push(warrior.uuid().clone());
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.contestants_ids.len() >= self.max_contestants
    }

    fn gen_random_pairs(&mut self) -> Vec<(Uuid, Uuid)> {
        let mut contestants_count = self.contestants_ids.len();
        dbg!(contestants_count);
        let nb_fights = contestants_count / 2;
        dbg!(&nb_fights);

        let mut pairs: Vec<(Uuid, Uuid)> = vec![];
        let mut i = 0;
        while i < nb_fights {
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let blue_corner = self.contestants_ids.swap_remove(random_index);
            contestants_count -= 1;
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let red_corner = self.contestants_ids.swap_remove(random_index);
            contestants_count -= 1;
            pairs.push((red_corner, blue_corner));
            i += 1;
        }
        pairs
    }

    pub fn auto(&mut self) -> Result<(), TournamentError> {
        let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        let mut round = 0;
        dbg!(&self.contestants_ids);
        let mut len = self.contestants_ids.len();
        while len > 1 {
            println!("Start Round: {}", round + 1);
            let pairs = self.gen_random_pairs();
            for pair in pairs {
                dbg!(&pair);
                let warrior1 = repo.get_by_uuid(&pair.0)?;
                let warrior2 = repo.get_by_uuid(&pair.1)?;
                let result = Fight::new(warrior1, warrior2).auto();
                match result.kind() {
                    FightResultKind::Victory(fighters) => {
                        repo.update(fighters.winner().uuid(), fighters.winner())?;
                        repo.update(fighters.loser().uuid(), fighters.loser())?;
                        self.contestants_ids.push(fighters.winner().uuid().clone());
                    },
                    FightResultKind::Tie((warrior1, warrior2)) => {
                        repo.update(warrior1.uuid(), warrior1)?;
                        repo.update(warrior2.uuid(), warrior2)?;
                    }
                }
            }
            len = self.contestants_ids.len();
            round += 1;
        }
        Ok(())
    }

    pub fn release_warriors(&self) -> Result<(), TournamentError> {
        let warriors_repository : FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
        for warrior_uuid in &self.contestants_ids {
            let mut warrior = warriors_repository.get_by_uuid(&warrior_uuid)?;
            warrior.set_current_tournament(None);
            warriors_repository.update(&warrior_uuid, &warrior)?;
        }
        Ok(())
    }
}

impl HasName for Tournament {
    fn name(&self) -> &Name {
        &self.name
    }
}

impl GenRandom for Tournament {
    fn gen_random() -> Self {
        let pow = rand::thread_rng().gen_range(1..10);
        let mut max_contestants = 2;
        let mut i = 0;
        while i < pow {
            max_contestants *= 2;
            i += 1;
        }
        Self::new(
            TournamentNameDictionary::new().get_random_item(),
            max_contestants
        )
    }
}

impl UniqueEntity for Tournament {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_not_accept_more_than_max_contestants() {
        let mut tournament = Tournament {
            uuid: Uuid::new_v4(),
            name: TournamentNameDictionary::new().get_random_item(),
            max_contestants: 1,
            contestants_ids: vec![Uuid::new_v4()],
        };
        let warrior = Warrior::gen_random();
        let result = tournament.add_contestant(&warrior);
        assert!(!result.is_ok())
    }

    #[test]
    fn should_add_warrior_uuid_when_accepting() {
        let mut tournament = Tournament {
            uuid: Uuid::new_v4(),
            name: TournamentNameDictionary::new().get_random_item(),
            max_contestants: 2,
            contestants_ids: vec![],
        };
        let mut expected_uuids: Vec<Uuid> = vec![];
        let warrior = Warrior::gen_random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);

        let warrior = Warrior::gen_random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);
    }
}
