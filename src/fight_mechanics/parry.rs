use super::fight_action::{ExecuteFightActionResult, ShowFightActionResult};
use super::critical_parry::CriticalParry;
use crate::warrior::Warrior;
use super::{TakeDamage, RollDamage, TakeReducibleDamage};
pub enum ParryAttemptResult {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}

impl ShowFightActionResult for ParryAttemptResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self {
            ParryAttemptResult::CriticalSuccess => println!("{} parried perfectly", victim.name()),
            ParryAttemptResult::Success => println!("{} parried", victim.name()),
            ParryAttemptResult::Failure => println!("{} hits {}", assailant.name(), victim.name()),
            ParryAttemptResult::CriticalFailure => println!("{} failed to parry miserably", victim.name())
        }
    }
}

impl ExecuteFightActionResult for ParryAttemptResult {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) {
        match self {
            ParryAttemptResult::CriticalFailure => victim.take_damage(assailant.roll_damage() * 2),
            ParryAttemptResult::Failure => victim.take_reduced_damage(assailant.roll_damage()),
            ParryAttemptResult::Success => {},
            ParryAttemptResult::CriticalSuccess => {
                let mut critical_parry_result = victim.critical_parry(assailant);
                critical_parry_result.show_fight_action_result(assailant, victim);
                critical_parry_result.execute(assailant, victim);
            }
        }
    }
}