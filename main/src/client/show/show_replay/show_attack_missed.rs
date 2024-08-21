use shared::assault::attack_missed::AttackMissed;

use super::ShowReplay;

impl ShowReplay for AttackMissed {
    fn show_replay(
        &self,
        assailant: &dyn crate::client::show::ReplayActor,
        _: &dyn crate::client::show::ReplayActor,
        _: &shared::assault::assault_consequence::AssaultConsequences
    ) -> String {
        format!("{} missed his attack", assailant.show_self())
    }
}
