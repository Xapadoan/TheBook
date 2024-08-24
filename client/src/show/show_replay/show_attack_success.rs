use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::attack_success::AttackSuccess;

use crate::show::ReplayActor;

use super::ShowReplay;

impl ShowReplay for AttackSuccess {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        format!("{} hits {}", assailant.show_self(), victim.show_self())
    }
}
