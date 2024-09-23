use shared::{name::Name, tournament::Fighter};

use super::ShowSelf;

impl ShowSelf for Fighter {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}