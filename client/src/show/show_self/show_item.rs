use shared::inventory::Item;

use super::{ShowSelf, ShowSelfExtended};

impl ShowSelf for Item {
    fn show_self(&self) -> String {
        match self {
            Item::Protection(protection) => protection.show_self(),
            Item::Weapon(weapon) => weapon.show_self(),
        }
    }
}

impl ShowSelfExtended for Item {
    fn show_self_extended(&self) -> String {
        match self {
            Item::Protection(protection) => protection.show_self_extended(),
            Item::Weapon(weapon) => weapon.show_self_extended(),
        }
    }
}
