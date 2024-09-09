use std::error::Error; 
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct InventoryError<T: Debug> {
    message: String,
    context: T,
}

#[derive(Debug)]
pub enum InventoryErrorKind {
    NotAWeapon,
    NotAProtection,
    ItemNotFound,
}

impl<T: Debug> InventoryError<T> {
    pub fn new(kind: &InventoryErrorKind, context: T) -> Self {
        match kind {
            InventoryErrorKind::NotAProtection => Self {
                message: "Not a protection".to_string(),
                context,
            },
            InventoryErrorKind::NotAWeapon => Self {
                message: "Not a weapon".to_string(),
                context,
            },
            InventoryErrorKind::ItemNotFound => Self {
                message: "Item not found".to_string(),
                context,
            }
        }
    }
}

impl<T: Debug> Display for InventoryError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nContext:\n{:?}", self.message, self.context)
    }
}

impl<T: Debug> Error for InventoryError<T> {}
