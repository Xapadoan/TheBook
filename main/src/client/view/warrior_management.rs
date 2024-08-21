use std::path::PathBuf;

use shared::health::IsDead;
use shared::name::Name;
use shared::player::Player;
use shared::random::Random;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{MutableWarriorCollection, Warrior, WarriorCollection};
use uuid::Uuid;

use crate::client::prompt::prompt_bool;
use crate::client::select_warrior::select_warrior;
use crate::client::show::ShowFightReplay;
use crate::repository::{FileRepository, PlayerRepository, Repository};
use crate::tournament::replay::manager::ReplayManager;

use super::view_error::ViewError;

pub fn returning_warriors(player: &mut Player) -> Result<(), ViewError> {
    let mut map = ReplayManager::map_warriors_to_replays(player)?;
    let warriors_repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
    for (tournament_uuid, warriors) in map.iter_mut() {
        let replay_manager = ReplayManager::new(tournament_uuid);
        let tournament = replay_manager.get_tournament_replay()?;
        let show_replay = prompt_bool(&format!(
            "The {} tournament ended, {} of your warriors were there do you want to see what happened ?",
            tournament.name(),
            warriors.len(),
        ))?;
        if !show_replay {
            return Ok(());
        }

        'show_warrior: loop {
            let warrior = select_warrior(warriors)?;
            if warrior.is_none() {
                break 'show_warrior;
            }
            let warrior = warrior.unwrap();
            let number_of_rounds = tournament.number_of_rounds();
            show_warrior_tournament(warrior, number_of_rounds, &replay_manager)?;
            warrior.set_current_tournament(None);
            warriors_repo.update(warrior.uuid(), warrior)?;
        }
    }
    replace_dead_warriors(player)?;
    Ok(())
}

fn show_warrior_tournament(
    warrior: &Warrior,
    number_of_rounds: usize,
    replay_manager: &ReplayManager,
) -> Result<(), ViewError> {
    let mut round_index: u8 = 0;
    let mut warrior_lost = false;
    while !warrior_lost && usize::from(round_index) < number_of_rounds {
        let fight_summary = replay_manager.get_fight_summary_for_warrior(warrior, round_index)?;
        let prompt: String = if fight_summary.winner().is_some_and(|uuid| &uuid == warrior.uuid()) {
            format!(
                "{} won the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index,
            )
        } else if fight_summary.loser().is_some_and(|uuid| &uuid == warrior.uuid()) {
            warrior_lost = true;
            format!(
                "{} lost the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index,
            )
        } else {
            warrior_lost = true;
            format!(
                "Both warriors were eliminated during the {}th round, do you want to see a replay of the fight ?",
                round_index,
            )
        };
        let show_fight_replay = prompt_bool(&prompt)?;
        if show_fight_replay {
            let fight_replay = replay_manager.get_fight_replay(&fight_summary)?;
            let (mut warrior1, mut warrior2) = replay_manager.get_fight_warriors(&fight_summary)?;
            fight_replay.show_fight_replay((&mut warrior1, &mut warrior2));
        }
        round_index += 1;
    }
    Ok(())
}

fn replace_dead_warriors(player: &mut Player) -> Result<(), ViewError> {
    let mut dead_warriors_uuids: Vec<Uuid> = vec![];
    for warrior in player.warriors() {
        if warrior.is_dead() {
            dead_warriors_uuids.push(warrior.uuid().clone());
        }
    }
    for uuid in dead_warriors_uuids {
        if let Some(w) = player.take_warrior(&uuid) {
            println!("{} died during the last tournament", w.name());
            let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
            repo.delete(w.uuid())?;
        }
        let w = Warrior::random();
        println!("{} will join your team", w.name());
        player.warriors_mut().push(w);
    }
    let repo = PlayerRepository::build()?;
    repo.update(player.uuid(), player)?;
    Ok(())
}
