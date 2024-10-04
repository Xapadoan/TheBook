use shared::equipment::{rupture::Rupture, weapon::Weapon};
use shared::name::Name;
use shared::stats::StatModifier;

use super::{ShowSelf, ShowSelfExtended};

impl ShowSelf for Weapon {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}

impl ShowSelfExtended for Weapon {
    fn show_self_extended(&self) -> String {
        let mut str = self.show_self();
        if self.is_sharp() {
            str += " Sh";
        } else {
            str += " Bl";
        }

        if self.is_two_handed() {
            str += " - 2h";
        } else {
            str += " - 1h";
        }
        
        
        str += format!(
            " DMG: {} RUP: {} AT: {} PRD: {} COU: {}",
            self.additional_damages(),
            match self.rupture() { Some(rup) => rup.to_string(), None => "None".to_string() },
            self.attack_mod(),
            self.parry_mod(),
            self.courage_mod(),
        ).as_str();

        str
    }
}
