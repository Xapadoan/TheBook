use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::attack_missed::AttackMissed;

use crate::show::ReplayActor;

use super::ShowReplay;

impl ShowReplay for AttackMissed {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        _: &dyn ReplayActor,
        _: &AssaultConsequences
    ) -> String {
        format!("{} missed his attack", assailant.show_self())
    }
}
