use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::parry_not_possible::ParryNotPossible;

use super::{ShowAction, ShowSelf, TournamentReplayActor};

impl ShowAction for ParryNotPossible {
    fn show_action(
        &self,
        _: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        match self {
            ParryNotPossible::HasNoWeapon => format!(
                "{} has no weapon thus can not parry",
                victim.show_self(),
            ),
            ParryNotPossible::IsDead => format!(
                "{} is dead and can not parry",
                victim.show_self(),
            ),
            ParryNotPossible::IsUnconscious => format!(
                "{} is unconscious and can not parry",
                victim.show_self(),
            ),
            ParryNotPossible::MustMissParry(misses) => format!(
                "{} can't parry because he is {}",
                victim.show_self(),
                misses.reason().show_self(),
            ),
        }
    }
}
