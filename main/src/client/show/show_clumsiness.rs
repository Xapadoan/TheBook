use shared::assault::clumsiness::Clumsiness;

use super::{ShowAction, TournamentReplayActor};

impl ShowAction for Clumsiness {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
        format!("{} missed miserably", assailant.show_self())
    }
}
