use std::error::Error; 
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct ShopManagerError {
    message: String,
    context: String,
}

#[derive(Debug)]
pub enum ShopManagerErrorKind {
    ReadError,
}

impl ShopManagerError {
    pub fn new(kind: &ShopManagerErrorKind, context: String) -> Self {
        match kind {
            ShopManagerErrorKind::ReadError => Self { message: "Read Error".to_string(), context, },
        }
    }
}

impl Display for ShopManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nContext:\n{:?}", self.message, self.context)
    }
}

impl Error for ShopManagerError {}
