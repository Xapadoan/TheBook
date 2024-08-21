use super::{Player, PlayerBuildError};

// server only
pub trait PlayerBuilder {
    fn get_username(&mut self) -> Result<(), PlayerBuildError>;
    fn get_display_name(&mut self) -> Result<(), PlayerBuildError>;
    fn get_warriors(&mut self) -> Result<(), PlayerBuildError>;
    fn build(self) -> Player;
}
