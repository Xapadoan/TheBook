use shared::assault::assailant::Assailant;
use shared::tournament::Fighter;

use super::ShowSelf;

pub trait ReplayActor: Assailant + ShowSelf {}
impl ReplayActor for Fighter {}

pub trait AssaultReplay {
    fn assault_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
    ) -> String;
}
