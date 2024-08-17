use shared::assault::assailant::Assailant;
use shared::assault::assault_consequence::AssaultConsequences;
use shared::warrior::Warrior;

pub trait TournamentReplayActor: Assailant + ShowSelf {}
impl TournamentReplayActor for Warrior {}
pub trait ShowAction {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
        consequences: &AssaultConsequences
    ) -> String;
}

pub trait AssaultReplay {
    fn assault_replay(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
    ) -> String;
}

pub trait ShowSelf {
    fn show_self(&self) -> String;
}