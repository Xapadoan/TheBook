use shared::assault::attack_clumsiness::AttackClumsiness;
use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::clumsiness::Clumsiness;

use crate::client::show::show_resolution::{show_lose_eye, show_lose_finger, show_rupture};
use crate::client::show::show_self_critical_hit::show_self_critical_hit;
use crate::client::show::{ReplayActor, ShowSelf};

use super::ShowReplay;

impl ShowReplay for AttackClumsiness {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        consequences: &AssaultConsequences,
    ) -> String {
        match self.kind() {
            Clumsiness::RegularFail => String::from(""),
            Clumsiness::Fall => format!("{} mixes his feet, trips and fall down", assailant.show_self()),
            Clumsiness::DropWeapon => format!(
                "{}'s {} hits {}'s {} and slips from his hands.",
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
            ),
            Clumsiness::BreakWeapon => {
                let rupture_damages = consequences.for_assailant().weapon_damages().unwrap();
                let weapon = assailant.weapon().as_ref().unwrap();
                format!(
                    "{} swigs his {} but misses and hits the floor heavily. It {}",
                    assailant.show_self(),
                    weapon.show_self(),
                    show_rupture(weapon, rupture_damages)
                )
            },
            Clumsiness::HitSelf => format!(
                "{}'s foot slips and he hit himself while trying to balance",
                assailant.show_self(),
            ),
            Clumsiness::CriticalHitSelf => {
                let self_critical = consequences.for_assailant().self_critical_hit().as_ref().unwrap();
                format!(
                    "{} trips and fall right on his {}. As a result, {}",
                    assailant.show_self(),
                    assailant.weapon().as_ref().unwrap().show_self(),
                    show_self_critical_hit(self_critical, assailant, consequences.for_assailant()),
                )
            },
            Clumsiness::LoseEye => format!(
                "{}'s {} somehow bounced back and ended right in his eye. {}",
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
                show_lose_eye(consequences.for_assailant().injury())
            ),
            Clumsiness::LoseFinger => format!(
                "{} starts falling down, but manages to catchup with {}'s {}, one of his finger nicely sliding along the blade. {}",
                assailant.show_self(),
                victim.show_self(),
                victim.weapon().as_ref().unwrap().show_self(),
                show_lose_finger(consequences.for_assailant().injury())
            ),
        }
    }
}
