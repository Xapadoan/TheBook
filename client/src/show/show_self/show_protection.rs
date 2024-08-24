use shared::equipment::protection::Protection;
use shared::name::Name;

use super::ShowSelf;

impl ShowSelf for Protection {
    fn show_self(&self) -> String {
        self.name().to_string()
    }
}