use shared::assault::critical_parry::CriticalParry;

use super::{ShowAction, TournamentReplayActor};

impl ShowAction for CriticalParry {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
        format!("{} parried like a boss", victim.show_self())
    }
}