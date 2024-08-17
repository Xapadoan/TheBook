use shared::assault::attack_clumsiness::AttackClumsiness;
use shared::assault::parry_clumsiness::ParryClumsiness;
use shared::warrior::body::injury::Injury;
use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::clumsiness::Clumsiness;

use crate::client::show::show_resolution::show_rupture;
use crate::client::show::show_self_critical_hit::show_self_critical_hit;

use super::show_resolution::show_lose_eye;
use super::{ShowAction, ShowSelf, TournamentReplayActor};

impl ShowAction for AttackClumsiness {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
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
                show_lose_finger_resolution(consequences.for_assailant().injury())
            ),
        }
    }
}

fn show_lose_finger_resolution(possible_injury: &Option<Injury>) -> String {
    match possible_injury {
        Some(_) => String::from(""),
        None => String::from("Luckily, he already lost the finger involved before"),
    }
}

impl ShowAction for ParryClumsiness {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
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
                show_lose_finger_resolution(consequences.for_assailant().injury())
            ),
        }
    }
}
