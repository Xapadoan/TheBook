use shared::temporary_handicap::TemporaryHandicapReason;

use super::ShowSelf;

impl ShowSelf for TemporaryHandicapReason {
    fn show_self(&self) -> String {
        match self {
            TemporaryHandicapReason::FellDown => String::from("on the floor"),
            TemporaryHandicapReason::LostBalance => String::from("trying to balance"),
        }
    }
}