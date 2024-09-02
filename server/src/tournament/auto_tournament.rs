use rand::Rng;
use shared::inventory::Inventory;
use shared::tournament::{Tournament, TournamentError};
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;
use std::path::PathBuf;

use crate::replay::{
    FightReplayBuilder,
    FightReplayBuilderError,
    RoundReplayBuilder,
    RoundReplayBuilderError,
    TournamentReplayBuilder,
    TournamentReplayBuilderError,
};
use crate::repository::{FileRepository, Repository, RepositoryError};

use super::fight::{Fight, FightError};


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
    fn gen_random_pairs(&mut self, remaining_contestants_ids: &mut Vec<Uuid>) -> Vec<(Uuid, Uuid)>;
    fn auto(&mut self) -> Result<(), TournamentError>;
}

impl AutoTournament for Tournament {
    fn gen_random_pairs(&mut self, remaining_contestants_ids: &mut Vec<Uuid>) -> Vec<(Uuid, Uuid)> {
        // let mut contestants_count = self.number_of_contestants();
        // let mut contestants_ids = self.contestants_ids();
        let mut count = remaining_contestants_ids.len();
        let nb_fights = count / 2;

        let mut pairs: Vec<(Uuid, Uuid)> = vec![];
        let mut i = 0;
        while i < nb_fights {
            let random_index = rand::thread_rng().gen_range(0..count);
            let blue_corner = remaining_contestants_ids.swap_remove(random_index);
            count -= 1;
            let random_index = rand::thread_rng().gen_range(0..count);
            let red_corner = remaining_contestants_ids.swap_remove(random_index);
            count -= 1;
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
        let mut remaining_contestants_ids = self.contestants_ids();
        while remaining_contestants_ids.len() > 1 {
            let mut round_replay_builder = RoundReplayBuilder::build(
                self.uuid(),
                round_index,
            )?;
            let pairs = self.gen_random_pairs(&mut remaining_contestants_ids);
            for pair in pairs {
                let mut fight_replay_builder = FightReplayBuilder::build(self.uuid())?;
                let mut warrior1 = repo.get_by_uuid(&pair.0)?;
                let mut warrior1_dropped_items = Inventory::new();
                let mut warrior2 = repo.get_by_uuid(&pair.1)?;
                let mut warrior2_dropped_items = Inventory::new();
                fight_replay_builder.record_warriors_init_state(&warrior1, &warrior2)?;
                let result = Fight::auto(
                    &mut fight_replay_builder,
                    &mut warrior1,
                    &mut warrior1_dropped_items,
                    &mut warrior2,
                    &mut warrior2_dropped_items,
                )?;
                self.add_dropped_items(warrior1.uuid(), warrior1_dropped_items);
                self.add_dropped_items(warrior2.uuid(), warrior2_dropped_items);
                fight_replay_builder.write_turn_summaries()?;
                repo.update(warrior1.uuid(), &warrior1)?;
                repo.update(warrior2.uuid(), &warrior2)?;
                if let Some(winner) = result.winner() {
                    remaining_contestants_ids.push(winner.clone())
                }
                round_replay_builder.push_summary(result);
            }
            round_replay_builder.write_summaries()?;
            round_index += 1;
        }
        Ok(())
    }
}
