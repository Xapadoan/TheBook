use std::{collections::HashMap, u32};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::item::{Item, MutableItems};

const INVENTORY_MAX_SLOTS: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    gold: u32,
    items: HashMap<Uuid, Item>
}

impl Inventory {
    pub fn new() -> Self {
        Self { gold: 0, items: HashMap::new() }
    }

    pub fn items(&self) -> &HashMap<Uuid, Item> {
        &self.items
    }

    // server only
    pub fn items_mut(&mut self) -> &mut HashMap<Uuid, Item> {
        &mut self.items
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    // server only
    fn set_gold(&mut self, gold: u32) {
        self.gold = gold;
    }
    
    // server only
    pub fn add_gold(&mut self, gold: u32) {
        match self.gold.checked_add(gold) {
            Some(amount) => self.set_gold(amount),
            None => self.set_gold(u32::MAX)
        }
    }

    // server only
    pub fn remove_gold(&mut self, gold: u32) {
        match self.gold.checked_sub(gold) {
            Some(amount) => self.set_gold(amount),
            None => self.set_gold(0),
        }
    }

    // server only
    pub fn join(&mut self, mut inventory: Inventory) {
        self.add_gold(inventory.gold());
        for (_, item) in inventory.items.drain() {
            self.add_item(item);
        }
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
