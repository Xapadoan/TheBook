use shared::equipment::weapon::Weapon;
use shared::name::Name;

use super::ShowSelf;

impl ShowSelf for Weapon {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}
