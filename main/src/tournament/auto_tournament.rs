use rand::Rng;
use shared::tournament::{Tournament, TournamentError};
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;
use std::path::PathBuf;

use crate::repository::{FileRepository, Repository, RepositoryError};
use crate::tournament::fight::FightResultKind;
use crate::tournament::replay::tournament_replay::TournamentReplayBuilder;

use super::fight::{Fight, FightError};
use super::replay::fight_replay::{FightReplayBuilder, FightReplayBuilderError};
use super::replay::round_replay::{RoundReplayBuilder, RoundReplayBuilderError};
use super::replay::tournament_replay::TournamentReplayBuilderError;


impl From<RepositoryError> for TournamentError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Tournament Repository Error:\n{value}"))
    }
}

impl From<FightError> for TournamentError {
    fn from(value: FightError) -> Self {
        Self::new(format!("Fight Error:\n{value}"))
    }
}

impl From<RoundReplayBuilderError> for TournamentError {
    fn from(value: RoundReplayBuilderError) -> Self {
        Self::new(format!("Round Replay Builder Error:\n{value}"))
    }
}

impl From<FightReplayBuilderError> for TournamentError {
    fn from(value: FightReplayBuilderError) -> Self {
        Self::new(format!("Fight Replay Builder Error:\n{value}"))
    }
}

impl From<TournamentReplayBuilderError> for TournamentError {
    fn from(value: TournamentReplayBuilderError) -> Self {
        Self::new(format!("Fight Replay Builder Error:\n{value}"))
    }
}

pub trait AutoTournament {
    fn gen_random_pairs(&mut self) -> Vec<(Uuid, Uuid)>;
    fn auto(&mut self) -> Result<(), TournamentError>;
}

impl AutoTournament for Tournament {
    // pub fn add_contestant(&mut self, warrior: &dyn TournamentContestant) -> Result<(), TournamentError> {
    //     if self.contestants_ids.len() + 1 > self.max_contestants {
    //         return Err(TournamentError::new(String::from("Tournament will not allow more contestants")));
    //     }
    //     self.contestants_ids.push(warrior.uuid().clone());
    //     Ok(())
    // }

    fn gen_random_pairs(&mut self) -> Vec<(Uuid, Uuid)> {
        let mut contestants_count = self.number_of_contestants();
        let nb_fights = contestants_count / 2;

        let mut pairs: Vec<(Uuid, Uuid)> = vec![];
        let mut i = 0;
        while i < nb_fights {
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let blue_corner = self.contestants_ids_mut().swap_remove(random_index);
            contestants_count -= 1;
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let red_corner = self.contestants_ids_mut().swap_remove(random_index);
            contestants_count -= 1;
            pairs.push((red_corner, blue_corner));
            i += 1;
        }
        pairs
    }

    fn auto(&mut self) -> Result<(), TournamentError> {
        let tournament_replay_builder = TournamentReplayBuilder::build(self.uuid())?;
        tournament_replay_builder.write_tournament_init_state(&self)?;
        let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
        let mut round_index = 0;
        let mut len = self.number_of_contestants();
        while len > 1 {
            let mut round_replay_builder = RoundReplayBuilder::build(
                self.uuid(),
                round_index,
            )?;
            let pairs = self.gen_random_pairs();
            for pair in pairs {
                let mut fight_replay_builder = FightReplayBuilder::build(self.uuid())?;
                let warrior1 = repo.get_by_uuid(&pair.0)?;
                let warrior2 = repo.get_by_uuid(&pair.1)?;
                fight_replay_builder.record_warriors_init_state(&warrior1, &warrior2)?;
                let result = Fight::new(
                    warrior1,
                    warrior2
                ).auto(&mut fight_replay_builder)?;
                fight_replay_builder.write_turn_summaries()?;
                round_replay_builder.push_summary(fight_replay_builder.replay_uuid(), &result);
                match result.kind() {
                    FightResultKind::Victory(fighters) => {
                        repo.update(fighters.winner().uuid(), fighters.winner())?;
                        repo.update(fighters.loser().uuid(), fighters.loser())?;
                        self.contestants_ids_mut().push(fighters.winner().uuid().clone());
                    },
                    FightResultKind::Tie((warrior1, warrior2)) => {
                        repo.update(warrior1.uuid(), &warrior1)?;
                        repo.update(warrior2.uuid(), &warrior2)?;
                    }
                }
            }
            round_replay_builder.write_summaries()?;
            len = self.number_of_contestants();
            round_index += 1;
        }
        Ok(())
    }
}
