use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::random::Random;

use super::assailant::Assailant;
use super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder};
use super::attack_attempt::AttackAttemptResult;
use super::attack_clumsiness::AttackClumsiness;
use super::attack_missed::AttackMissed;
use super::attack_success::AttackSuccess;
use super::critical_hit::CriticalHit;
use super::critical_parry::CriticalParry;
use super::not_possible::NotPossible;
use super::parry_attempt::ParryAttemptResult;
use super::parry_clumsiness::ParryClumsiness;
use super::parry_success::ParrySuccess;
// use super::assault_result::AssaultResult;

// pub struct AssaultSummary<T: for<'a> AssaultResult<'a>> {
#[derive(Debug, Serialize, Deserialize)]
pub struct AssaultSummary {
    assailant_uuid: Uuid,
    victim_uuid: Uuid,
    not_possible: Option<NotPossible>,
    attack_clumsiness: Option<AttackClumsiness>,
    attack_missed: Option<AttackMissed>,
    attack_success: Option<AttackSuccess>,
    parry_clumsiness: Option<ParryClumsiness>,
    parry_success: Option<ParrySuccess>,
    parry_critical: Option<CriticalParry>,
    attack_critical: Option<CriticalHit>,
    consequences: AssaultConsequences
}

impl AssaultSummary {
    pub fn assailant_uuid(&self) -> &Uuid {
        &self.assailant_uuid
    }

    pub fn victim_uuid(&self) -> &Uuid {
        &self.victim_uuid
    }

    pub fn not_possible(&self) -> &Option<NotPossible> {
        &self.not_possible
    }

    pub fn attack_clumsiness(&self) -> &Option<AttackClumsiness> {
        &self.attack_clumsiness
    }

    pub fn attack_missed(&self) -> &Option<AttackMissed> {
        &self.attack_missed
    }

    pub fn attack_success(&self) -> &Option<AttackSuccess> {
        &self.attack_success
    }

    pub fn parry_clumsiness(&self) -> &Option<ParryClumsiness> {
        &self.parry_clumsiness
    }

    pub fn parry_success(&self) -> &Option<ParrySuccess> {
        &self.parry_success
    }

    pub fn parry_critical(&self) -> &Option<CriticalParry> {
        &self.parry_critical
    }

    pub fn attack_critical(&self) -> &Option<CriticalHit> {
        &self.attack_critical
    }

    pub fn consequences(&self) -> &AssaultConsequences {
        &self.consequences
    }

    pub fn new(assailant: &dyn Assailant, victim: &dyn Assailant) -> Self {
        let not_possible = assailant.can_attack(victim);
        let mut attack_clumsiness = None;
        let mut attack_missed = None;
        let mut attack_success = None;
        let mut parry_clumsiness = None;
        let mut parry_success = None;
        let mut parry_critical = None;
        let mut attack_critical = None;
        let consequences = if let Some(inner) = &not_possible {
            inner.to_consequences(assailant, victim)
        } else  {
            match assailant.attack_attempt() {
                AttackAttemptResult::CriticalFailure => {
                    attack_clumsiness = Some(AttackClumsiness::random());
                    attack_clumsiness.as_ref().unwrap().to_consequences(assailant, victim)
                },
                AttackAttemptResult::Failure => {
                    attack_missed = Some(AttackMissed::new());
                    attack_missed.as_ref().unwrap().to_consequences(assailant, victim)
                },
                AttackAttemptResult::Success => {
                    attack_success = Some(AttackSuccess::new());
                    match victim.parry_attempt() {
                        ParryAttemptResult::CriticalFailure => {
                            parry_clumsiness = Some(ParryClumsiness::random());
                            parry_clumsiness.as_ref().unwrap().to_consequences(assailant, victim)
                        },
                        ParryAttemptResult::Failure => attack_success.as_ref().unwrap().to_consequences(assailant, victim),
                        ParryAttemptResult::Success => {
                            parry_success = Some(ParrySuccess::new());
                            parry_success.as_ref().unwrap().to_consequences(assailant, victim)
                        },
                        ParryAttemptResult::CriticalSuccess => {
                            parry_critical = Some(victim.deal_critical_parry());
                            parry_critical.as_ref().unwrap().to_consequences(assailant, victim)
                        },
                    }
                },
                AttackAttemptResult::CriticalSuccess => {
                    attack_critical = Some(assailant.deal_critical_hit());
                    attack_critical.as_ref().unwrap().to_consequences(assailant, victim)
                }
            }
        };
        Self {
            assailant_uuid: assailant.uuid().clone(),
            victim_uuid: victim.uuid().clone(),
            not_possible,
            attack_clumsiness,
            attack_missed,
            attack_success,
            parry_clumsiness,
            parry_success,
            parry_critical,
            attack_critical,
            consequences,
        }
    }
}
