use shared::{name::Name, warrior::Warrior};

use super::ShowSelf;

impl ShowSelf for Warrior {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}