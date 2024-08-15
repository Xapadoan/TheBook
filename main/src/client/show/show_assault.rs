use shared::assault::{assault_summary::AssaultSummary, attack_missed::AttackMissed, attack_success::AttackSuccess};

use super::{ShowAction, TournamentReplayActor};

impl ShowAction for AssaultSummary {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
        if let Some(impossible) = self.not_possible() {
            return impossible.show_action(assailant, victim);
        }
        if let Some(clumsiness) = self.attack_clumsiness() {
            return clumsiness.kind().show_action(assailant, victim);
        }
        if let Some(miss) = self.attack_missed() {
            return miss.show_action(assailant, victim);
        }
        if let Some(attack) = self.attack_success() {
            let mut str = attack.show_action(assailant, victim);
            if let Some(clumsiness) = self.parry_clumsiness() {
                str += format!(". When trying to parry, {}", clumsiness.kind().show_action(assailant, victim)).as_str();
            }
            if let Some(_) = self.parry_success() {
                str += ", but he managed to parry the blow";
            }
            if let Some(critical) = self.parry_critical() {
                str += critical.show_action(assailant, victim).as_str();
            }
            return str;
        }
        if let Some(critical) = self.attack_critical() {
            return critical.show_action(assailant, victim)
        }
        dbg!(&self);
        return String::from("???");
    }
}

impl ShowAction for AttackMissed {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, _: &dyn TournamentReplayActor) -> String {
        format!("{} missed his attack", assailant.show_self())
    }
}

impl ShowAction for AttackSuccess {
    fn show_action(&self, assailant: &dyn TournamentReplayActor, victim: &dyn TournamentReplayActor) -> String {
        format!("{} hits {}", assailant.show_self(), victim.show_self())
    }
}
