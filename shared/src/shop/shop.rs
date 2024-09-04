use serde::{Deserialize, Serialize};

use crate::inventory::{HasInventory, HasMutableInventory, Inventory};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    inventory: Inventory,
}

impl Shop {
    pub fn new(inventory: Inventory) -> Self {
        Self { inventory }
    }
}

impl HasInventory for Shop {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}

impl HasMutableInventory for Shop {
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}
