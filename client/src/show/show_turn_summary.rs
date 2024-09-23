use std::collections::HashMap;

use shared::equipment::weapon::OptionalMutableWeapon;
use shared::assault::attack_attempt::AttackThreshold;
use shared::assault::parry_attempt::ParryThreshold;
use shared::health::MutableHealth;
use shared::name::Name;
use shared::tournament::Fighter;
use shared::assault::assault_summary::AssaultSummary;
use shared::assault::common_traits::TakeDamage;
use shared::assault::end_turn_consequences::EndTurnConsequences;
use shared::replay::turn_summary::TurnSummary;
use shared::unique_entity::UniqueEntity;

use super::{AssaultReplay, ReplayActor, ShowSelf};

pub trait ShowTurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Fighter,
        red_corner: &mut Fighter,
    ) -> String;
}

impl ShowTurnSummary for TurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Fighter,
        red_corner: &mut Fighter,
    ) -> String {
        let assaults = self.assaults();
        let mut roles = get_roles(
            &assaults[0],
            blue_corner,
            red_corner,
        );
        let assailant = roles.remove("assailant").unwrap();
        let victim = roles.remove("victim").unwrap();
        let mut str = format!("{}", assaults[0].assault_replay(assailant, victim));
        assaults[0].consequences().apply(
            assailant,
            victim,
        );

        let mut roles = get_roles(
            &assaults[1],
            blue_corner,
            red_corner,
        );
        let assailant = roles.remove("assailant").unwrap();
        let victim = roles.remove("victim").unwrap();
        str = format!("{}\n{}", str, assaults[1].assault_replay(assailant, victim));
        assaults[1].consequences().apply(
            assailant,
            victim,
        );

        let blue_turn_end_str = show_end_turn(self.blue_turn_end(), blue_corner);
        if !blue_turn_end_str.is_empty() {
            str = format!("{}\n{}", str, blue_turn_end_str);
        }
        blue_corner.take_damage(self.blue_turn_end().duration_damages());

        let red_turn_end_str = show_end_turn(self.red_turn_end(), red_corner);
        if !red_turn_end_str.is_empty() {
            str = format!("{}\n{}", str, red_turn_end_str);
        }
        red_corner.take_damage(self.red_turn_end().duration_damages());

        str += "\n\n";
        str += display_fighters(blue_corner, red_corner).as_str();
        str += "\n";

        str
    }
}

fn display_fighters(blue_corner: &Fighter, red_corner: &Fighter) -> String {
    let mut str = String::new();
    str += format!("{}\t\t\t\t{}\n", blue_corner.name(), red_corner.name()).as_str();
    str += format!(
        "{}/{}\t\t\t\t{}/{}\n",
        blue_corner.health().current(),
        blue_corner.health().max(),
        red_corner.health().current(),
        red_corner.health().max(),
    ).as_str();
    str += format!(
        "{}\t\t\t{}\n",
        blue_corner.weapon().show_self(),
        red_corner.weapon().show_self(),
    ).as_str();
    str += format!(
        "AT: {}\t\t\t\tAT: {}\n",
        blue_corner.attack_threshold(),
        red_corner.attack_threshold(),
    ).as_str();
    str += format!(
        "PRD: {}\t\t\t\tPRD: {}",
        blue_corner.parry_threshold(),
        red_corner.parry_threshold(),
    ).as_str();
    str
}

fn get_roles<'a>(
    assault: &AssaultSummary,
    blue_corner: &'a mut Fighter,
    red_corner: &'a mut Fighter,
) -> HashMap<&'static str, &'a mut Fighter> {
    let mut roles = HashMap::new();
    if assault.assailant_uuid() == blue_corner.uuid() {
        roles.insert("assailant", blue_corner);
        roles.insert("victim", red_corner);
    } else {
        roles.insert("assailant", red_corner);
        roles.insert("victim", blue_corner);
    }
    roles
}

fn show_end_turn(end: &EndTurnConsequences, victim: &dyn ReplayActor) -> String {
    let damages = end.duration_damages();
    if damages > 0 {
        format!("{} lost {} hp from duration damages", damages, victim.show_self())
    } else {
        String::from("")
    }
}
