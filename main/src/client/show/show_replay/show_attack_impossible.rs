use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::attack_not_possible::AttackNotPossible;

use crate::client::show::{ReplayActor, ShowSelf};

use super::ShowReplay;

impl ShowReplay for AttackNotPossible {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        match self {
            AttackNotPossible::AssailantHasNoWeapon => format!(
                "{} has no weapon thus can not attack",
                assailant.show_self(),
            ),
            AttackNotPossible::AssailantIsDead => format!(
                "{} is dead and can not attack",
                assailant.show_self(),
            ),
            AttackNotPossible::AssailantIsUnconscious => format!(
                "{} is unconscious and can not attack",
                assailant.show_self(),
            ),
            AttackNotPossible::AssailantMustMissAssault(misses) => format!(
                "{} can't attack because he is {}",
                assailant.show_self(),
                misses.reason().show_self(),
            ),
            AttackNotPossible::VictimHasNoWeapon => format!(
                "{} has no weapon and must surrender",
                victim.show_self(),
            ),
            AttackNotPossible::VictimIsDead => format!(
                "{} is dead",
                victim.show_self(),
            ),
            AttackNotPossible::VictimIsUnconscious => format!(
                "{} is unconscious",
                victim.show_self(),
            ),
        }
    }
}