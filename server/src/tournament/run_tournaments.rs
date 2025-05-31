use chrono::Utc;
use shared::{
    equipment::weapon::Weapon, inventory::Inventory, name::Name, player::{Player, PlayerBuilder}, random::Random, tournament::Tournament, unique_entity::UniqueEntity, warrior::{Warrior, WarriorCollection}
};

use crate::{repository::{sql_repository::{players::CreatePlayerSchema, players_inventories::{CreatePlayerInventorySlotSchema, PlayersInventoriesRepository, UpdatePlayerInventorySchema}, tournaments::UpdateTournamentSchema, tournaments_warriors::TournamentsWarriorsRepository, warriors::{CreateWarriorSchema, UpdateWarriorSchema}, weapons::CreateWeaponSchema}, RepositoryCreate, RepositoryDelete, RepositoryList, RepositoryRead, RepositoryUpdate}, tournament::{auto_tournament::AutoTournament, bot_player_builder::BotPlayerBuilder}, warrior::WarriorManager};

use super::manager::TournamentManagerError;

pub async fn run_tournaments<'a, T, W, P, We>(
    tournaments_repo: &T,
    warriors_repo: &W,
    players_repo: &P,
    players_inventories_repo: &PlayersInventoriesRepository<'a>,
    tournaments_warriors_repo: &TournamentsWarriorsRepository<'a>,
    weapons_repo: &We
) -> Result<(), TournamentManagerError>
where
    T: RepositoryList<Tournament> +
        RepositoryUpdate<Tournament, UpdateTournamentSchema>,
    W: RepositoryCreate<Warrior, CreateWarriorSchema> +
        RepositoryRead<Warrior> +
        RepositoryUpdate<Warrior, UpdateWarriorSchema>,
    P: RepositoryCreate<Player, CreatePlayerSchema> +
        RepositoryRead<Player> +
        RepositoryDelete,
    We: RepositoryCreate<Weapon, CreateWeaponSchema>,
{
    let warriors_manager = WarriorManager::new(warriors_repo);
    let mut available_tournaments = tournaments_repo.list().await?;
    for tournament in available_tournaments.iter_mut() {
        eprintln!("[DEBUG] Running tournament {} ({})", tournament.name(), tournament.uuid().to_string());
        let contestants = tournaments_warriors_repo.list_contestants(tournament.uuid())
            .await?;
        if let Err(_) = tournament.set_contestants(contestants) {
            eprintln!("[ERROR] tournament {} could not run", tournament.uuid());
        }
        warriors_manager.apply_passive_healing(&tournament.contestants_ids()).await?;
        eprintln!("[WARN] Bot Player should be deleted no matter what happens, this is not the case");
        let bot_player = gen_bot_player(tournament, players_repo, warriors_repo, weapons_repo).await?;
        tournaments_repo.update(tournament.uuid(), &UpdateTournamentSchema { started_at: Utc::now() }).await?;
        tournament.auto(warriors_repo).await?;
        for (player_uuid, contestants) in tournament.contestants().clone() {
            let mut gains_for_player = Inventory::new();
            let player_inventory = players_inventories_repo.read_for_player(&player_uuid).await?;
            for warrior_uuid in contestants {
                if let Some(inventory) = tournament.take_contestant_inventory(&warrior_uuid) {
                    gains_for_player.join(inventory);
                }
            }

            let gained_items: Vec<CreatePlayerInventorySlotSchema> = gains_for_player.items()
                .values()
                .map(|gained_item| {
                    CreatePlayerInventorySlotSchema::from_item(player_inventory.uuid(), &gained_item)
                })
                .collect();
            players_inventories_repo.update_for_player(&player_uuid, UpdatePlayerInventorySchema {
                gold: player_inventory.gold() + gains_for_player.gold(),
                items_to_create: gained_items,
            }).await?;
        }
        players_repo.delete(bot_player.uuid()).await?;
        // self.repo.delete(&uuid).await?;
    }
    Ok(())
}

async fn gen_bot_player<P, W, We>(
    tournament: &mut Tournament,
    players_repo: &P,
    warriors_repo: &W,
    weapons_repo: &We,
) -> Result<Player, TournamentManagerError>
where
    P: RepositoryCreate<Player, CreatePlayerSchema>,
    W: RepositoryCreate<Warrior, CreateWarriorSchema>,
    We: RepositoryCreate<Weapon, CreateWeaponSchema>,
{
    let mut bot_builder = BotPlayerBuilder::new(tournament);
    bot_builder.build_username()?;
    bot_builder.build_display_name()?;
    bot_builder.build_warriors()?;
    let bot_player = bot_builder.build();
    let player_schema = CreatePlayerSchema::new(bot_player.username());
    if let Err(e) = players_repo.create(&player_schema).await {
        eprintln!("[ERROR] Failed to create bot player for tournament {}", tournament.uuid());
        return Err(TournamentManagerError::from(e))
    }
    for warrior in bot_player.warriors() {
        let weapon = Weapon::random();
        let weapon_schema = CreateWeaponSchema::from(weapon);
        let weapon = weapons_repo.create(&weapon_schema).await?;
        let schema = CreateWarriorSchema::new(warrior.clone(), &player_schema.uuid, weapon.uuid());
        let bot_warrior = warriors_repo.create(&schema).await?;
        eprintln!("[DEBUG] Adding bot warrior {} to tournament {}", schema.uuid, tournament.uuid());
        tournament.add_contestant(bot_player.uuid(), &bot_warrior)?;
    }
    Ok(bot_player)
}
