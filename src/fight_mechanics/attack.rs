use super::{fight_action::{ApplyFightActionResult, ShowFightActionResult}, parry::ParryAttemptResult, CanMissParries};
use crate::warrior::Warrior;
use super::{ParryAttempt, CriticalHitOn};

pub enum AttackAttemptResult {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

impl ShowFightActionResult for AttackAttemptResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self {
            AttackAttemptResult::CriticalFailure => println!("{} missed miserably", assailant.name()),
            AttackAttemptResult::Failure => println!("{} missed", assailant.name()),
            AttackAttemptResult::Success => println!("{} is landing a hit on {}", assailant.name(), victim.name()),
            AttackAttemptResult::CriticalSuccess => println!("{}'s life is in danger", victim.name()),
        }
    }
}

impl ApplyFightActionResult for AttackAttemptResult {
    fn apply_fight_action_result(&self, assailant: &mut Warrior, victim: &mut Warrior) {
        if victim.must_miss_parry() {
            victim.miss_parry();
        }
        match self {
            AttackAttemptResult::CriticalFailure => {},
            AttackAttemptResult::Failure => {},
            AttackAttemptResult::Success => {
                if victim.must_miss_parry() {
                    ParryAttemptResult::Failure.show_fight_action_result(assailant, victim);
                    ParryAttemptResult::Failure.apply_fight_action_result(assailant, victim);
                    return;
                }
                let parry_attempt_result = victim.parry_attempt();
                parry_attempt_result.show_fight_action_result(assailant, victim);
                parry_attempt_result.apply_fight_action_result(assailant, victim);
            },
            AttackAttemptResult::CriticalSuccess => {
                let crit_consequence = assailant.critical_hit_on(victim);
                crit_consequence.show_fight_action_result(assailant, victim);
                crit_consequence.apply_fight_action_result(assailant, victim);
            }
        }
    }
}
