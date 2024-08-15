use shared::assault::not_possible::NotPossible;

use super::{ShowAction, TournamentReplayActor};

impl ShowAction for NotPossible {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
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
            NotPossible::AssailantMustMissAssault(_) => format!(
                "{} misses this assault",
                assailant.show_self(),
            ),
            NotPossible::VictimHasNoWeapon => format!(
                "{} has no weapon thus can not attack",
                victim.show_self(),
            ),
            NotPossible::VictimIsDead => format!(
                "{} is dead and can not attack",
                victim.show_self(),
            ),
            NotPossible::VictimIsUnconscious => format!(
                "{} is unconscious and can not attack",
                victim.show_self(),
            ),
        }
    }
}