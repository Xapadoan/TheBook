use shared::assault::critical_hit::CriticalHit;

use super::{ShowAction, TournamentReplayActor};

impl ShowAction for CriticalHit {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
        format!("{} hits like a boss", assailant.show_self())
    }
}
