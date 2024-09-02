use serde::{Deserialize, Serialize};

use crate::equipment::{protection::Protection, weapon::Weapon};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Item {
    Weapon(Weapon),
    Protection(Protection),
}

pub trait Items {
    fn items(&self) -> &Vec<Item>;
}

pub trait MutableItems: Items {
    fn add_item(&mut self, item: Item) -> Option<Item>;
    fn remove_item(&mut self, index: usize) -> Option<Item>;
}