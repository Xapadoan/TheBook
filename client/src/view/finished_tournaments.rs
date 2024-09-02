use shared::auth::Session;
use shared::health::IsDead;
use shared::name::Name;
use shared::player::Player;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use server::api;
use crate::prompt::{prompt_bool, swap_select_with_keys};
use crate::show::ShowWarriorFightReplay;

use super::view_error::ViewError;

pub fn returning_warriors(session: &Session) -> Result<(), ViewError> {
    let player = api::players::read(session.uuid())?;
    let mut map = api::replay::available_replays(player.uuid())?;
    for (tournament_uuid, warrior_uuids) in map.iter_mut() {
        let tournament = api::replay::tournament_replay(tournament_uuid)?;
        let show_replay = prompt_bool(&format!(
            "The {} tournament ended, {} of your warriors were there do you want to see what happened ?",
            tournament.name(),
            warrior_uuids.len(),
        ))?;
        if !show_replay {
            return Ok(());
        }

        let mut warriors: Vec<&Warrior> = player.warriors()
            .iter()
            .filter(|w| warrior_uuids.contains(w.uuid()))
            .collect();
        'show_warrior: loop {
            let warrior = swap_select_with_keys(
                "Witch warrior's tournament do you want to replay ?",
                &mut warriors,
                |warrior: &Warrior| { warrior.name().to_string() }
            )?;
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
        } else if fight_summary.winner().is_some_and(|uuid| &uuid != warrior.uuid()) {
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
            fight_replay.show_warrior_fight_replay((&mut warrior1, &mut warrior2), warrior.uuid());
        }
        round_index += 1;
    }
    Ok(())
}

fn replace_dead_warriors(player: Player) -> Result<(), ViewError> {
    for warrior in player.warriors() {
        if warrior.is_dead() {
            println!(
                "{} died during the last tournament, all his items have been sent to your inventory",
                warrior.name(),
            );
            api::players::remove_warrior(player.uuid(), warrior.uuid())?;
            let new_warrior = api::players::gen_random_warrior(player.uuid())?;
            println!("{} will join your team", new_warrior.name());
        }
    }
    Ok(())
}
