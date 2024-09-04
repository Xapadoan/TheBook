use std::{error::Error, fmt::{Debug, Display}};

use shared::shop::Shop;

use super::{ShopManager, ShopManagerError};

pub fn read_shop() -> Result<Shop, ShopAPIError> {
    let shop = ShopManager::read_shop()?;
    Ok(shop)
}

#[derive(Debug)]
pub struct ShopAPIError {
    message: String,
    context: String,
}

#[derive(Debug)]
pub enum ShopAPIErrorKind {
    ShopManagerError(ShopManagerError),
}

impl ShopAPIError {
    pub fn new(kind: &ShopAPIErrorKind) -> Self {
        match kind {
            ShopAPIErrorKind::ShopManagerError(e) => Self { message: "Shop API Error".to_string(), context: e.to_string() },
        }
    }
}

impl Display for ShopAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nContext:\n{}", self.message, self.context)
    }
}

impl Error for ShopAPIError {}

impl From<ShopManagerError> for ShopAPIError {
    fn from(value: ShopManagerError) -> Self {
        Self::new(&ShopAPIErrorKind::ShopManagerError(value))
    }
}