use crate::random::Random;

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder};
use super::attack_attempt::AttackAttemptResult;
use super::attack_clumsiness::AttackClumsiness;
use super::attack_missed::AttackMissed;
use super::attack_success::AttackSuccess;
use super::critical_parry::CriticalParry;
use super::parry_attempt::ParryAttemptResult;
use super::parry_clumsiness::ParryClumsiness;
use super::parry_success::ParrySuccess;

pub struct Assault {}

impl Assault {
    pub fn new(assailant: &dyn Assailant, victim: &dyn Assailant) -> AssaultConsequences {
        if let Some(attack_not_possible) = assailant.can_attack(victim) {
            return attack_not_possible.to_consequences(assailant, victim);
        }
        match assailant.attack_attempt() {
            AttackAttemptResult::CriticalFailure => AttackClumsiness::random().to_consequences(assailant, victim),
            AttackAttemptResult::Failure => AttackMissed::new().to_consequences(assailant, victim),
            AttackAttemptResult::Success => match victim.parry_attempt() {
                ParryAttemptResult::CriticalFailure => ParryClumsiness::random().to_consequences(assailant, victim),
                ParryAttemptResult::Failure => AttackSuccess::new().to_consequences(assailant, victim),
                ParryAttemptResult::Success => ParrySuccess::new().to_consequences(assailant, victim),
                ParryAttemptResult::CriticalSuccess => CriticalParry::random().to_consequences(assailant, victim),
            },
            AttackAttemptResult::CriticalSuccess => assailant.deal_critical_hit().to_consequences(assailant, victim),
        }
    }
}
