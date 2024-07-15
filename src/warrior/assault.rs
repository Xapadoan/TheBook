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
pub mod show_action;
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

impl AssaultResult {
    pub fn assailant_uuid(&self) -> &Uuid {
        &self.assailant_uuid
    }
}

impl ApplyDamageSummary for AssaultResult {
    fn apply_damage_summary<T: TakeDamage>(&self, assailant: &mut T, victim: &mut T) {
        self.damage_summary.apply_damage_summary(assailant, victim)
    }
}

impl ShowAction for AssaultResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
        where
            A: crate::name::HasName + super::weapon::MayHaveWeapon + super::temporary_handicap::assaults_miss::CanMissAssaults,
            V: crate::name::HasName + super::weapon::MayHaveWeapon + super::body::HasBody + super::temporary_handicap::parries_miss::CanMissParries {
        self.attack.show(assailant, victim);
        if self.parry.is_some() {
            self.parry.as_ref().unwrap().show(assailant, victim)
        }
        self.damage_summary.show(assailant, victim)
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
