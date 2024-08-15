use uuid::Uuid;

use crate::unique_entity::UniqueEntity;

pub trait TournamentContestant: UniqueEntity {
    fn current_tournament(&self) -> &Option<Uuid>;
    fn set_current_tournament(&mut self, tournament_uuid: Option<Uuid>);
}
