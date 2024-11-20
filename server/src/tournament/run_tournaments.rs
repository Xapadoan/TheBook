use chrono::{DateTime, Utc};
use shared::{inventory::{HasMutableInventory, Inventory}, name::Name, player::{Player, PlayerBuilder}, tournament::Tournament, unique_entity::UniqueEntity, warrior::{Warrior, WarriorCollection}};
use uuid::Uuid;

use crate::{repository::{sql_repository::{players::CreatePlayerSchema, players_inventories::{CreatePlayerInventorySlotSchema, PlayersInventoriesRepository, UpdatePlayerInventorySchema}, tournaments::UpdateTournamentSchema, warriors::{CreateWarriorSchema, UpdateWarriorSchema}}, RepositoryCreate, RepositoryDelete, RepositoryList, RepositoryRead, RepositoryUpdate}, tournament::{auto_tournament::AutoTournament, bot_player_builder::BotPlayerBuilder}, warrior::WarriorManager};

use super::manager::TournamentManagerError;

pub async fn run_tournaments<'a, T, W, P>(
    tournaments_repo: &T,
    warriors_repo: &W,
    players_repo: &P,
    players_inventories_repo: &PlayersInventoriesRepository<'a>,
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
{
    panic!("Not implemented");
    eprintln!("[ERROR] No filter on tournaments");
    let warriors_manager = WarriorManager::new(warriors_repo);
    let available_tournaments = tournaments_repo.list().await?;
    for tournament in available_tournaments {
        eprintln!("[DEBUG] Running tournament {} ({})", tournament.name(), tournament.uuid().to_string());
        warriors_manager.apply_passive_healing(&tournament.contestants_ids()).await?;
        eprintln!("[WARN] Bot Player should be deleted no matter what happens, this is not the case");
        let bot_player = gen_bot_player(&mut tournament, players_repo).await?;
        tournaments_repo.update(tournament.uuid(), &UpdateTournamentSchema { started_at: Utc::now() }).await?;
        tournament.auto(warriors_repo).await?;
        for (player_uuid, contestants) in tournament.contestants().clone() {
            let mut gains_for_player = Inventory::new();
            let mut player_inventory = players_inventories_repo.read_for_player(&player_uuid).await?;
            for warrior_uuid in contestants {
                if let Some(inventory) = tournament.take_contestant_inventory(&warrior_uuid) {
                    // player_inventory.join(inventory);
                    gains_for_player.join(inventory);
                }
            }
            let gained_items: Vec<CreatePlayerInventorySlotSchema> = gains_for_player.items()
                .into_values()
                .map(|gained_item| {
                    CreatePlayerInventorySlotSchema::from_item(player_inventory.uuid(), &gained_item)
                })
                .collect();
            players_inventories_repo.update_for_player(&player_uuid, UpdatePlayerInventorySchema {
                gold: gains_for_player.gold(),
                items_to_create: gained_items,
            }).await?;
        }
        players_repo.delete(bot_player.uuid()).await?;
        // self.repo.delete(&uuid).await?;
    }
    Ok(())
}

async fn gen_bot_player<P>(tournament: &mut Tournament, bots_repo: &P) -> Result<Player, TournamentManagerError>
where
    P: RepositoryCreate<Player, CreatePlayerSchema>
{
    panic!("Not implemented");
    let mut bot_builder = BotPlayerBuilder::new(tournament);
    bot_builder.build_username()?;
    bot_builder.build_display_name()?;
    bot_builder.build_warriors().await?;
    let bot = bot_builder.build();
    for warrior in bot.warriors() {
        tournament.add_contestant(bot.uuid(), warrior)?;
    }
    let bot = CreatePlayerSchema::new(bot.username());
    let created_bot = bots_repo.create(&bot).await?;
    Ok(created_bot)
}

// async fn delete_bot_player(bot_uuid: &Uuid, bo) -> Result<(), TournamentManagerError> {
//     panic!("Not implemented")
//     let bots_repo = PlayerRepository::build()?;
//     bots_repo.delete(bot_uuid)?;
//     Ok(())
// }