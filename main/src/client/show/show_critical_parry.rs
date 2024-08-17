use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::critical_parry::CriticalParry;

use crate::client::show::show_self_critical_hit::show_self_critical_hit;

use super::show_resolution::show_rupture;
use super::{ShowAction, ShowSelf, TournamentReplayActor};

impl ShowAction for CriticalParry {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
        consequences: &AssaultConsequences,
    ) -> String {
        match self {
            CriticalParry::AssailantBreaksWeapon => {
                let weapon = assailant.weapon().as_ref().unwrap();
                let rupture_damages = consequences.for_assailant().weapon_damages().unwrap();
                format!(
                    "{} stomps on {}'s {} and it {}",
                    victim.show_self(),
                    assailant.show_self(),
                    weapon.show_self(),
                    show_rupture(weapon, rupture_damages)
                )
            },
            CriticalParry::AssailantCriticalHit => {
                let critical_hit = consequences.for_assailant().counter_critical_hit().as_ref().unwrap();
                let reversed = AssaultConsequences::reversed(consequences);
                format!(
                    "at the last moment, {}",
                    critical_hit.show_action(victim, assailant, &reversed),
                )
            },
            CriticalParry::AssailantDropsWeapon => format!(
                "{} hits {}'s {} so hard he drops it",
                victim.show_self(),
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
            ),
            CriticalParry::AssailantFalls => format!(
                "{} pushed {} back so hard he fell down",
                victim.show_self(),
                assailant.show_self(),
            ),
            CriticalParry::AssailantHit => format!(
                "{} manage to counter the blow and hit {}",
                victim.show_self(),
                assailant.show_self(),
            ),
            CriticalParry::AssailantRepelled => format!(
                "{} repelled him back",
                victim.show_self()
            ),
            CriticalParry::AssailantSelfCriticalHit => {
                let critical_hit = consequences.for_assailant().self_critical_hit().as_ref().unwrap();
                format!(
                    "{} managed to push back {}'s {} against him and {}",
                    victim.show_self(),
                    assailant.show_self(),
                    assailant.weapon().as_ref().unwrap().show_self(),
                    show_self_critical_hit(critical_hit, assailant, consequences.for_assailant())
                )
            },
            CriticalParry::AssailantTrips => format!(
                "{} throws {} off-balance",
                victim.show_self(),
                assailant.show_self(),
            ),
            CriticalParry::RegularParry => format!(
                "{} parries the blow majestically",
                victim.show_self(),
            ),
        }
    }
}