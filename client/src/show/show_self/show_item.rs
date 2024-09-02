use shared::inventory::Item;

use super::ShowSelf;

impl ShowSelf for Item {
    fn show_self(&self) -> String {
        match self {
            Item::Protection(protection) => protection.show_self(),
            Item::Weapon(weapon) => weapon.show_self(),
        }
    }
}
