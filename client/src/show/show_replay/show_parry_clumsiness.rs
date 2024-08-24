use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::clumsiness::Clumsiness;
use shared::assault::parry_clumsiness::ParryClumsiness;

use crate::show::show_resolution::{show_lose_eye, show_lose_finger, show_rupture};
use crate::show::show_self_critical_hit::show_self_critical_hit;
use crate::show::{ReplayActor, ShowSelf};

use super::ShowReplay;

impl ShowReplay for ParryClumsiness {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        consequences: &AssaultConsequences
    ) -> String {
        match self.kind() {
            Clumsiness::RegularFail => format!(
                "{} almost lost his grip on his {}",
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
            ),
            Clumsiness::Fall => format!("{} mixes his feet, trips and falls down", victim.show_self()),
            Clumsiness::DropWeapon => format!(
                "{} loses his grip on his {} and it slips from his hands.",
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
            ),
            Clumsiness::BreakWeapon => {
                let rupture_damages = consequences.for_victim().weapon_damages().unwrap();
                let weapon = victim.weapon().as_ref().unwrap();
                format!(
                    "{} used the handle of his {}. It {}",
                    victim.show_self(),
                    weapon.show_self(),
                    show_rupture(weapon, rupture_damages)
                )
            },
            Clumsiness::HitSelf => format!(
                "{}'s {} is pushed back by {} against his chest",
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
                assailant.show_self(),
            ),
            Clumsiness::CriticalHitSelf => {
                let self_critical = consequences.for_victim().self_critical_hit().as_ref().unwrap();
                format!(
                    "{} trips and fall right on his {}. As a result, he {}",
                    victim.show_self(),
                    victim.weapon().as_ref().unwrap().show_self(),
                    show_self_critical_hit(self_critical, victim, consequences.for_victim()),
                )
            },
            Clumsiness::LoseEye => format!(
                "{}'s {} somehow bounced back and ended right in his eye. {}",
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
                show_lose_eye(consequences.for_assailant().injury())
            ),
            Clumsiness::LoseFinger => format!(
                "{}'s {} stroke right on one of {}'s fingers. {}",
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
                victim.show_self(),
                show_lose_finger(consequences.for_assailant().injury())
            ),
        }
    }
}