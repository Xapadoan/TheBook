use shared::health::IsDead;
use shared::name::Name;
use shared::player::Player;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{MutableWarriorCollection, Warrior, WarriorCollection};
use uuid::Uuid;

use crate::api;
use crate::prompt::prompt_bool;
use crate::select_warrior::select_warrior;
use crate::show::ShowFightReplay;

use super::view_error::ViewError;

pub fn returning_warriors(player: &mut Player) -> Result<(), ViewError> {
    let mut map = api::replay::available_replays(player.uuid())?;
    for (tournament_uuid, warrior_uuids) in map.iter() {
        let tournament = api::replay::tournament_replay(tournament_uuid)?;
        let show_replay = prompt_bool(&format!(
            "The {} tournament ended, {} of your warriors were there do you want to see what happened ?",
            tournament.name(),
            warrior_uuids.len(),
        ))?;
        if !show_replay {
            return Ok(());
        }

        let mut warriors: Vec<&Warrior> = vec![];
        for warrior in player.warriors() {
            if warrior_uuids.contains(warrior.uuid()) {
                warriors.push(&warrior);
            }
        }
        'show_warrior: loop {
            let warrior = select_warrior(&mut warriors)?;
            if warrior.is_none() {
                break 'show_warrior;
            }
            let warrior = warrior.unwrap();
            let number_of_rounds = tournament.number_of_rounds();
            show_warrior_tournament(tournament_uuid, warrior, number_of_rounds)?;
            api::tournaments::remove_contestant(warrior.uuid())?;
        }
    }
    replace_dead_warriors(player)?;
    Ok(())
}

fn show_warrior_tournament(
    tournament_uuid: &Uuid,
    warrior: &Warrior,
    number_of_rounds: usize,
) -> Result<(), ViewError> {
    let mut round_index: u8 = 0;
    let mut warrior_lost = false;
    while !warrior_lost && usize::from(round_index) < number_of_rounds {
        let fight_summary = api::replay::fight_summary_for_warrior(
            tournament_uuid,
            warrior.uuid(),
            round_index,
        )?;
        let prompt: String = if fight_summary.winner().is_some_and(|uuid| &uuid == warrior.uuid()) {
            format!(
                "{} won the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index + 1,
            )
        } else if fight_summary.loser().is_some_and(|uuid| &uuid == warrior.uuid()) {
            warrior_lost = true;
            format!(
                "{} lost the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index + 1,
            )
        } else {
            warrior_lost = true;
            format!(
                "Both warriors were eliminated during the {}th round, do you want to see a replay of the fight ?",
                round_index + 1,
            )
        };
        let show_fight_replay = prompt_bool(&prompt)?;
        if show_fight_replay {
            let fight_replay = api::replay::fight_replay(tournament_uuid, &fight_summary)?;
            let (mut warrior1, mut warrior2) = api::replay::fight_warriors(tournament_uuid, &fight_summary)?;
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
            api::warriors::delete_warrior(w.uuid())?;
        }
        let w = api::players::gen_random_warrior(player.uuid())?;
        println!("{} will join your team", w.name());
        player.warriors_mut().push(w);
    }
    Ok(())
}
