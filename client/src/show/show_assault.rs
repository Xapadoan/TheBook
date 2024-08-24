use shared::assault::assault_summary::AssaultSummary;

use super::{AssaultReplay, ShowReplay, ReplayActor};

impl AssaultReplay for AssaultSummary {
    fn assault_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
    ) -> String {
        if let Some(impossible) = self.not_possible() {
            return impossible.show_replay(assailant, victim, self.consequences());
        }
        if let Some(clumsiness) = self.attack_clumsiness() {
            return clumsiness.show_replay(assailant, victim, self.consequences());
        }
        if let Some(miss) = self.attack_missed() {
            return miss.show_replay(assailant, victim, self.consequences());
        }
        if let Some(attack) = self.attack_success() {
            let mut str = attack.show_replay(assailant, victim, self.consequences());
            if let Some(impossible_parry) = self.parry_not_possible() {
                return format!(
                    "{}, {}",
                    attack.show_replay(assailant, victim, self.consequences()),
                    impossible_parry.show_replay(assailant, victim, self.consequences()),
                )
            }
            if let Some(clumsiness) = self.parry_clumsiness() {
                let clumsiness_display = clumsiness.show_replay(assailant, victim, self.consequences());
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
                    critical.show_replay(assailant, victim, self.consequences()),
                );
            }
            return str;
        }
        if let Some(critical) = self.attack_critical() {
            return critical.show_replay(assailant, victim, self.consequences())
        }
        dbg!(&self);
        return String::from("???");
    }
}
