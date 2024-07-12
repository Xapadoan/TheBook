use attack::attack_attempt::AttackAttemptResult;
use attack::{Attack, AttackResult};
use damage_summary::{ApplyDamageSummary, DamageSummary};
use execute_action::ExecuteAction;
use parry::{Parry, ParryResult};
use serde::{Deserialize, Serialize};
use show_action::ShowAction;
use uuid::Uuid;

use crate::repository::main::UniqueEntity;

use super::{TakeDamage, Warrior};

pub mod attack;
pub mod parry;
pub mod damage_summary;
mod show_action;
mod execute_action;
mod clumsiness;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssaultResult {
    assailant_uuid: Uuid,
    victim_uuid: Uuid,
    attack: AttackResult,
    parry: Option<ParryResult>,
    damage_summary: DamageSummary,
}

impl ApplyDamageSummary for AssaultResult {
    fn apply_damage_summary<T: TakeDamage>(&self, assailant: &mut T, victim: &mut T) {
        self.damage_summary.apply_damage_summary(assailant, victim)
    }
}

pub trait Assault {
    fn assault(&mut self, victim: &mut Warrior) -> AssaultResult;
}

impl Assault for Warrior {
    fn assault(&mut self, victim: &mut Warrior) -> AssaultResult {
        let mut attack = self.attack(victim);
        // attack.show(self, victim);
        let mut damage_summary = attack.execute(self, victim);
        let parry = match attack.attack_attempt() {
            Some(attack_attempt) => match attack_attempt {
                AttackAttemptResult::Success => {
                    let mut parry_result = victim.parry(self);
                    // parry_result.show(self, victim);
                    damage_summary.merge(
                        parry_result.execute(self, victim),
                        false,
                    );
                    Some(parry_result)
                },
                _ => None
            },
            None => None,
        };

        AssaultResult {
            assailant_uuid: self.uuid().clone(),
            victim_uuid: victim.uuid().clone(),
            attack,
            parry,
            damage_summary,
        }
    }
}
