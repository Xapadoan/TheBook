use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::assault_summary::AssaultSummary;
use shared::assault::attack_missed::AttackMissed;
use shared::assault::attack_success::AttackSuccess;

use super::{AssaultReplay, ShowAction, TournamentReplayActor};

impl AssaultReplay for AssaultSummary {
    fn assault_replay(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
    ) -> String {
        if let Some(impossible) = self.not_possible() {
            return impossible.show_action(assailant, victim, self.consequences());
        }
        if let Some(clumsiness) = self.attack_clumsiness() {
            return clumsiness.show_action(assailant, victim, self.consequences());
        }
        if let Some(miss) = self.attack_missed() {
            return miss.show_action(assailant, victim, self.consequences());
        }
        if let Some(attack) = self.attack_success() {
            let mut str = attack.show_action(assailant, victim, self.consequences());
            if let Some(clumsiness) = self.parry_clumsiness() {
                let clumsiness_display = clumsiness.show_action(assailant, victim, self.consequences());
                str = format!(
                    "{}. When trying to parry, {}",
                    str,
                    clumsiness_display,
                );
            }
            if let Some(_) = self.parry_success() {
                str += ", but he managed to parry the blow";
            }
            if let Some(critical) = self.parry_critical() {
                str = format!(
                    "{}, but {}",
                    str,
                    critical.show_action(assailant, victim, self.consequences()),
                );
            }
            return str;
        }
        if let Some(critical) = self.attack_critical() {
            return critical.show_action(assailant, victim, self.consequences())
        }
        dbg!(&self);
        return String::from("???");
    }
}

impl ShowAction for AttackMissed {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        _: &dyn TournamentReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        format!("{} missed his attack", assailant.show_self())
    }
}

impl ShowAction for AttackSuccess {
    fn show_action(
        &self,
        assailant: &dyn TournamentReplayActor,
        victim: &dyn TournamentReplayActor,
        _: &AssaultConsequences,
    ) -> String {
        format!("{} hits {}", assailant.show_self(), victim.show_self())
    }
}
