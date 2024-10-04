use shared::equipment::{protection::Protection, rupture::Rupture};
use shared::name::Name;
use shared::stats::StatModifier;

use super::{ShowSelf, ShowSelfExtended};

impl ShowSelf for Protection {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}

impl ShowSelfExtended for Protection {
    fn show_self_extended(&self) -> String {
        format!("{} (PR: {} RUP:{} COU: {} DEX: {})",
            self.show_self().as_str(),
            self.amount(),
            match self.rupture() { Some(rup) => rup.to_string(), None => "None".to_string() },
            self.courage_mod(),
            self.dexterity_mod(),
        )
    }
}