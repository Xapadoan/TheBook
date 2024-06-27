use super::attack::attack_attempt::AttackAttemptResult;
use super::attack::AttackResult;
use super::parry::ParryResult;

use crate::fight_mechanics::RollDamage;

// pub struct DamageSummary {
//     reducible_damage: u8,
//     irreducible_damage: u8,
// }

// pub struct AssaultDamageSummary {
//     to_assailant: {
//         reducible_damage: 0,
//     },
//     to_victim: u8,
// }

impl AssaultDamageSummary {
    pub fn new<A: RollDamage>(assailant: &A, attack: &AttackResult, parry: &ParryResult) -> Self {
        let reducible_damage_to_victim = match attack.attack_attempt() {
            Some(attack_attempt) => match attack_attempt {
                AttackAttemptResult::Success => match parry.parry_attempt() {
                    Some(parry_attempt) => {},
                    None => assailant.roll_damage(),
                },
                _ => 0,
            }
            None => 0,
        };
    }
    pub fn to_assailant(&self) -> u8 {
        self.to_assailant
    }
    pub fn to_victim(&self) -> u8 {
        self.to_victim
    }
}
