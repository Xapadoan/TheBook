use serde::{Deserialize, Serialize};

use super::item::{Item, Items, MutableItems};

const INVENTORY_MAX_SLOTS: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<Item>
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}

impl Items for Inventory {
    fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

impl MutableItems for Inventory {
    fn add_item(&mut self, item: Item) -> Option<Item> {
        if self.items.len() < INVENTORY_MAX_SLOTS {
            self.items.push(item);
            None
        } else {
            Some(item)
        }
    }

    fn remove_item(&mut self, index: usize) -> Option<Item> {
        if let None = self.items.get(index) {
            None
        } else {
            Some(self.items.swap_remove(index))
        }
    }
}

pub trait HasInventory {
    fn inventory(&self) -> &Inventory;
}

pub trait HasMutableInventory: HasInventory {
    fn inventory_mut(&mut self) -> &mut Inventory;
}
