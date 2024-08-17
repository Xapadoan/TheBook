use shared::assault::{assault_consequence::AssaultConsequences, not_possible::NotPossible};

use super::{ShowAction, ShowSelf, TournamentReplayActor};

impl ShowAction for NotPossible {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
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