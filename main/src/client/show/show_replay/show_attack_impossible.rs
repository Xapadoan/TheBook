use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::not_possible::NotPossible;

use crate::client::show::{ReplayActor, ShowSelf};

use super::ShowReplay;

impl ShowReplay for NotPossible {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        match self {
            NotPossible::AssailantHasNoWeapon => format!(
                "{} has no weapon thus can not attack",
                assailant.show_self(),
            ),
            NotPossible::AssailantIsDead => format!(
                "{} is dead and can not attack",
                assailant.show_self(),
            ),
            NotPossible::AssailantIsUnconscious => format!(
                "{} is unconscious and can not attack",
                assailant.show_self(),
            ),
            NotPossible::AssailantMustMissAssault(misses) => format!(
                "{} can't attack because he is {}",
                assailant.show_self(),
                misses.reason().show_self(),
            ),
            NotPossible::VictimHasNoWeapon => format!(
                "{} has no weapon and must surrender",
                victim.show_self(),
            ),
            NotPossible::VictimIsDead => format!(
                "{} is dead",
                victim.show_self(),
            ),
            NotPossible::VictimIsUnconscious => format!(
                "{} is unconscious",
                victim.show_self(),
            ),
        }
    }
}