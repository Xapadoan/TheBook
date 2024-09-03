use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::item::{Item, MutableItems};

const INVENTORY_MAX_SLOTS: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    items: HashMap<Uuid, Item>
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn items(&self) -> &HashMap<Uuid, Item> {
        &self.items
    }

    // server_only
    pub fn items_mut(&mut self) -> &mut HashMap<Uuid, Item> {
        &mut self.items
    }
}

impl MutableItems for Inventory {
    fn add_item(&mut self, item: Item) -> Option<Item> {
        if self.items.len() < INVENTORY_MAX_SLOTS {
            self.items.insert(Uuid::new_v4(), item);
            None
        } else {
            Some(item)
        }
    }

    fn remove_item(&mut self, id: &Uuid) -> Option<Item> {
        self.items.remove(id)
    }
}

pub trait HasInventory {
    fn inventory(&self) -> &Inventory;
}

pub trait HasMutableInventory: HasInventory {
    fn inventory_mut(&mut self) -> &mut Inventory;
}
