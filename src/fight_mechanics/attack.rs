use super::fight_action::{ExecuteFightActionResult, ShowFightActionResult};
use super::parry::ParryAttemptResult;
use super::CanMissParries;
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

impl ExecuteFightActionResult for AttackAttemptResult {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) {
        if victim.must_miss_parry() {
            victim.miss_parry();
        }
        match self {
            AttackAttemptResult::CriticalFailure => {},
            AttackAttemptResult::Failure => {},
            AttackAttemptResult::Success => {
                if victim.must_miss_parry() {
                    ParryAttemptResult::Failure.show_fight_action_result(assailant, victim);
                    ParryAttemptResult::Failure.execute(assailant, victim);
                    return;
                }
                let mut parry_attempt_result = victim.parry_attempt();
                parry_attempt_result.show_fight_action_result(assailant, victim);
                parry_attempt_result.execute(assailant, victim);
            },
            AttackAttemptResult::CriticalSuccess => {
                let mut crit_consequence = assailant.critical_hit_on(victim);
                crit_consequence.show_fight_action_result(assailant, victim);
                crit_consequence.execute(assailant, victim);
            }
        }
    }
}
