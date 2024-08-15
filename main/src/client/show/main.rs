use shared::assault::assailant::Assailant;
use shared::warrior::Warrior;

pub trait TournamentReplayActor: Assailant + ShowSelf {}
impl TournamentReplayActor for Warrior {}
pub trait ShowAction {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String;
}

pub trait ShowSelf {
    fn show_self(&self) -> &str;
}