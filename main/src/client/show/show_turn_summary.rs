use shared::assault::assault_summary::AssaultSummary;
use shared::assault::common_traits::TakeDamage;
use shared::assault::end_turn_consequences::EndTurnConsequences;
use shared::replay::turn_summary::TurnSummary;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use super::{AssaultReplay, TournamentReplayActor};

pub trait ShowTurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Warrior,
        red_corner: &mut Warrior,
    ) -> String;
}

impl ShowTurnSummary for TurnSummary {
    fn show_turn_summary(
        &self,
        blue_corner: &mut Warrior,
        red_corner: &mut Warrior,
    ) -> String {
        let assaults = self.assaults();
        let (assailant, victim) = get_roles(
            &assaults[0],
            (blue_corner, red_corner),
        );
        let mut str = format!("{}", assaults[0].assault_replay(assailant, victim));
        assaults[0].consequences().apply(assailant, victim);

        let (assailant, victim) = get_roles(
            &assaults[1],
            (blue_corner, red_corner),
        );
        str = format!("{}\n{}", str, assaults[1].assault_replay(assailant, victim));
        assaults[0].consequences().apply(assailant, victim);

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
        str
    }
}

fn get_roles<'a>(
    assault: &AssaultSummary,
    actors: (&'a mut Warrior, &'a mut Warrior)
) -> (&'a mut Warrior, &'a mut Warrior) {
    if assault.assailant_uuid() == actors.0.uuid() {
        (actors.0, actors.1)
    } else {
        (actors.1, actors.0)
    }
}

fn show_end_turn(end: &EndTurnConsequences, victim: &dyn TournamentReplayActor) -> String {
    let damages = end.duration_damages();
    if damages > 0 {
        format!("{} lost {} hp from duration damages", damages, victim.show_self())
    } else {
        String::from("")
    }
}
