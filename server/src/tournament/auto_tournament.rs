use rand::Rng;
use shared::experience::GainExperience;
use shared::inventory::Inventory;
use shared::tournament::{Fighter, Tournament, TournamentError};
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
use super::fight_reward::FightReward;


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
                let mut warrior2 = repo.get_by_uuid(&pair.1)?;
                fight_replay_builder.record_warriors_init_state(&warrior1, &warrior2)?;
                let mut fighter1 = Fighter::from(&warrior1);
                let mut fighter2 = Fighter::from(&warrior2);
                let result = Fight::auto(
                    &mut fight_replay_builder,
                    &mut fighter1,
                    &mut fighter2,
                )?;
                fight_replay_builder.write_turn_summaries()?;
                let inventory1 = fighter1.consume(&mut warrior1);
                let inventory2 = fighter2.consume(&mut warrior2);
                if let Some(winner_uuid) = result.winner() {
                    let mut fight_rewards = Inventory::new();
                    fight_rewards.add_gold(self.fight_reward(round_index as usize));
                    if warrior1.uuid() == winner_uuid {
                        warrior1.gain_xp(20);
                        self.add_to_contestant_inventory(warrior1.uuid(), fight_rewards);
                    } else {
                        warrior2.gain_xp(20);
                        self.add_to_contestant_inventory(warrior2.uuid(), fight_rewards);
                    }
                    remaining_contestants_ids.push(winner_uuid.clone())
                }
                self.add_to_contestant_inventory(warrior1.uuid(), inventory1);
                self.add_to_contestant_inventory(warrior2.uuid(), inventory2);
                repo.update(warrior1.uuid(), &warrior1)?;
                repo.update(warrior2.uuid(), &warrior2)?;
                round_replay_builder.push_summary(result);
            }
            round_replay_builder.write_summaries()?;
            round_index += 1;
        }
        Ok(())
    }
}
