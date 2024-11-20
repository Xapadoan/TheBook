use shared::{equipment::weapon::Weapon, inventory::{Inventory, Item, MutableItems}};
use sqlx::{mysql::{MySqlArguments, MySqlRow}, query::{Map, Query}, query_builder::QueryBuilder as SQLXQueryBuilder, MySql, Pool};
use uuid::Uuid;

use crate::repository::{sql_repository::{weapons::WeaponsQueryBuilder, QueryBuilder}, RepositoryError};

use super::{CreatePlayerInventorySchema, PlayerInventoryJoinedSlotModel, PlayerInventoryModel, UpdatePlayerInventorySchema};

pub struct PlayersInventoriesQueryBuilder;
impl PlayersInventoriesQueryBuilder {
    fn read_player_slots_query(player_uuid: &Uuid) -> Map<
        'static,
        MySql,
        impl FnMut(MySqlRow) -> Result<PlayerInventoryJoinedSlotModel, sqlx::Error>,
        MySqlArguments,
    > {
        sqlx::query_as!(
            PlayerInventoryJoinedSlotModel,
            "SELECT \
                players_inventories.gold AS gold,\
                player_inventory_slots.weapon_uuid AS weapon_uuid,\
                player_inventory_slots.protection_uuid AS protection_uuid \
            FROM players_inventories \
            INNER JOIN player_inventory_slots \
            ON players_inventories.uuid = player_inventory_slots.inventory_uuid \
            WHERE players_inventories.player_uuid = ?",
            player_uuid.to_string(),
        )
    }
    fn update_player_gold_query(player_uuid: &Uuid, gold: u32) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "UPDATE players_inventories SET gold = ? WHERE player_uuid = ?",
            gold,
            player_uuid.to_string(),
        )
    }
}
impl QueryBuilder for PlayersInventoriesQueryBuilder {
    type Model = PlayerInventoryModel;
    type CreateSchema = CreatePlayerInventorySchema;
    type UpdateSchema = UpdatePlayerInventorySchema;

    fn create_query(item: &Self::CreateSchema) -> Query<'static, MySql, MySqlArguments> {
        sqlx::query!(
            "INSERT INTO players_inventories (uuid, player_uuid, gold) VALUES (?, ?, ?)",
            item.uuid,
            item.player_uuid,
            item.gold,
        )
    }
    fn read_query(uuid: &uuid::Uuid) -> sqlx::query::Map<
        'static,
        sqlx::MySql,
        impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
        sqlx::mysql::MySqlArguments
    > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM players_inventories WHERE uuid = ?",
            uuid.to_string(),
        )
    }
    fn list_query() -> sqlx::query::Map<
            'static,
            sqlx::MySql,
            impl FnMut(sqlx::mysql::MySqlRow) -> Result<Self::Model, sqlx::Error>,
            sqlx::mysql::MySqlArguments
        > {
        sqlx::query_as!(
            Self::Model,
            "SELECT * FROM players_inventories",
        )
    }
    fn update_query(uuid: &uuid::Uuid, item: &Self::UpdateSchema) -> sqlx::query::Query<'static, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        sqlx::query!(
            "UPDATE players_inventories SET gold = ? WHERE uuid = ?",
            item.gold,
            uuid.to_string(),
        )
    }
    fn delete_query(uuid: &uuid::Uuid) -> sqlx::query::Query<'static, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        sqlx::query!(
            "DELETE FROM players_inventories WHERE uuid = ?",
            uuid.to_string(),
        )
    }
}

pub struct PlayersInventoriesRepository<'a> {
    db_pool: &'a Pool<MySql>
}
impl<'a> PlayersInventoriesRepository<'a> {
    pub fn new(db_pool: &'a Pool<MySql>) -> Self {
        Self { db_pool }
    }
    pub async fn read_for_player(&self, player_uuid: &Uuid) -> Result<Inventory, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;

        let slots = PlayersInventoriesQueryBuilder::read_player_slots_query(player_uuid)
            .fetch_all(&mut *trx)
            .await?;
        let mut inventory = Inventory::new();
        if slots.len() > 0 {
            inventory.add_gold(slots[0].gold);
        }

        for slot in slots {
            if let Some(weapon_uuid) = slot.weapon_uuid {
                let weapon_uuid = Uuid::parse_str(&weapon_uuid)?;
                let weapon = WeaponsQueryBuilder::read_query(&weapon_uuid)
                    .fetch_one(&mut *trx)
                    .await?;
                let weapon = Weapon::try_from(weapon)?;
                inventory.add_item(Item::Weapon(weapon));
            } else if let Some(protection_uuid) = slot.protection_uuid {
                let protection_uuid = Uuid::parse_str(&protection_uuid)?;
                let protection = WeaponsQueryBuilder::read_query(&protection_uuid)
                    .fetch_one(&mut *trx)
                    .await?;
                let protection = Weapon::try_from(protection)?;
                inventory.add_item(Item::Weapon(protection));
            } else {
                eprintln!("[WARN] Unknown slot model: {slot:?}")
            }
        }
        Ok(inventory)
    }
    pub async fn update_for_player(&self, player_uuid: &Uuid, item: UpdatePlayerInventorySchema) -> Result<Inventory, RepositoryError> {
        let mut trx = self.db_pool.begin().await?;

        PlayersInventoriesQueryBuilder::update_player_gold_query(player_uuid, item.gold)
            .execute(&mut * trx)
            .await?;

        if item.items_to_create.len() > 0 {
            let mut query = SQLXQueryBuilder::new("INSERT into player_inventory_slots (uuid, inventory_uuid, weapon_uuid, protection_uuid) VALUES ");
            query.push_values(item.items_to_create.into_iter(), |mut b, value| {
                b.push_bind(value.uuid)
                    .push_bind(value.inventory_uuid)
                    .push_bind(value.weapon_uuid)
                    .push_bind(value.protection_uuid);
            });

            query.build().execute(&mut * trx).await?;
        }
        trx.commit().await?;

        let updated_inventory = self.read_for_player(player_uuid).await?;
        Ok(updated_inventory)
    }
}