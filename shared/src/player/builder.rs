use super::{Player, PlayerBuildError};

// server only
pub trait PlayerBuilder {
    fn build_username(&mut self) -> Result<(), PlayerBuildError>;
    fn build_display_name(&mut self) -> Result<(), PlayerBuildError>;
    fn build_warriors(&mut self) -> Result<(), PlayerBuildError>;
    fn build_inventory(&mut self) -> Result<(), PlayerBuildError>;
    fn build(self) -> Player;
}
