use shared::assault::assault_consequence::AssaultConsequences;

use crate::show::ReplayActor;

pub trait ShowReplay {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        consequences: &AssaultConsequences
    ) -> String;
}
