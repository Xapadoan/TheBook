use shared::assault::assailant::Assailant;
use shared::warrior::Warrior;

use super::ShowSelf;

pub trait ReplayActor: Assailant + ShowSelf {}
impl ReplayActor for Warrior {}

pub trait AssaultReplay {
    fn assault_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
    ) -> String;
}
