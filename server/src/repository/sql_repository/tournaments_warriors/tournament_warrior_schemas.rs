use uuid::Uuid;

pub struct CreateTournamentWarriorSchema {
    pub uuid: Uuid,
    pub tournament_uuid: Uuid,
    pub player_uuid: Uuid,
    pub warrior_uuid: Uuid,
}
impl CreateTournamentWarriorSchema {
    pub fn new(
        tournament_uuid: Uuid,
        player_uuid: Uuid,
        warrior_uuid: Uuid,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            tournament_uuid,
            player_uuid,
            warrior_uuid,
        }
    }
}
