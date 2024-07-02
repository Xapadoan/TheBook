use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize, de};

pub type SavePathBuf = PathBuf;

pub trait SaveManager<T: Serialize + de::DeserializeOwned> {
    fn build(save_dir: SavePathBuf) -> Result<Self, Box<dyn Error>>
    where Self: Sized;
    fn save(&self, item: T, file_path: SavePathBuf) -> Result<SavePathBuf, Box<dyn Error>>;
    fn build_from_save<'de>(&self, file_path: &SavePathBuf) -> Result<T, Box<dyn Error>>;
}
