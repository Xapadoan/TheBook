use std::collections::HashMap;

use shared::{assault::assault_summary::AssaultSummary, inventory::Inventory};
use shared::assault::common_traits::TakeDamage;
use shared::assault::end_turn_consequences::EndTurnConsequences;
use shared::replay::turn_summary::TurnSummary;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use crate::character_sheet::CharacterSheet;

use super::{AssaultReplay, ShowSelf, ReplayActor};

pub trait ShowTurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Warrior,
        blue_corner_lost_items: &mut Inventory,
        red_corner: &mut Warrior,
        red_corner_lost_items: &mut Inventory,
    ) -> String;
}

impl ShowTurnSummary for TurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Warrior,
        blue_corner_lost_items: &mut Inventory,
        red_corner: &mut Warrior,
        red_corner_lost_items: &mut Inventory,
    ) -> String {
        let assaults = self.assaults();
        let mut roles = get_roles(
            &assaults[0],
            (blue_corner, blue_corner_lost_items),
            (red_corner, red_corner_lost_items),
        );
        let (assailant, assailant_dropped_items) = roles.remove("assailant").unwrap();
        let (victim, victim_dropped_items) = roles.remove("victim").unwrap();
        let mut str = format!("{}", assaults[0].assault_replay(assailant, victim));
        assaults[0].consequences().apply(
            assailant,
            assailant_dropped_items,
            victim,
            victim_dropped_items,
        );

        let mut roles = get_roles(
            &assaults[1],
            (blue_corner, blue_corner_lost_items),
            (red_corner, red_corner_lost_items),
        );
        let (assailant, assailant_dropped_items) = roles.remove("assailant").unwrap();
        let (victim, victim_dropped_items) = roles.remove("victim").unwrap();
        str = format!("{}\n{}", str, assaults[1].assault_replay(assailant, victim));
        assaults[1].consequences().apply(
            assailant,
            assailant_dropped_items,
            victim,
            victim_dropped_items,
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

        str = format!("{}\n\n{}", str, CharacterSheet::new(blue_corner).show_self());
        str = format!("{}\n\n{}", str, CharacterSheet::new(red_corner).show_self());

        str
    }
}

fn get_roles<'a>(
    assault: &AssaultSummary,
    blue_corner: (&'a mut Warrior, &'a mut Inventory),
    red_corner: (&'a mut Warrior, &'a mut Inventory),
) -> HashMap<&'static str, (&'a mut Warrior, &'a mut Inventory)> {
    let mut roles = HashMap::new();
    if assault.assailant_uuid() == blue_corner.0.uuid() {
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
