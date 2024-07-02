use std::io::{BufWriter, Read, Write};
use std::{io, path::PathBuf};
use std::error::Error;
use std::fs;

use serde::{Serialize, de};
use serde_json::Serializer;

use crate::gen_random::GenRandom;
use crate::save::save_manager::{SaveManager, SavePathBuf};

use super::Warrior;

pub struct WarriorSaveManager {
    save_dir: SavePathBuf,
}

impl WarriorSaveManager {
    fn new(save_dir: SavePathBuf) -> Self {
        Self {
            save_dir,
        }
    }
}

// impl SaveManager<Warrior> for WarriorSaveManager {
//     fn build(save_dir: SavePathBuf) -> Result<Self, Box<dyn std::error::Error>> {
//         let warrior_save_manager = Self::new(save_dir);
//         if !warrior_save_manager.save_dir.as_path().try_exists()? {
//             fs::create_dir(warrior_save_manager.save_dir.as_path())?;
//         }
//         Ok(warrior_save_manager)
//     }

//     fn save(&self, item: Warrior, file_path: SavePathBuf) -> Result<SavePathBuf, Box<dyn Error>> {
//         let mut full_path = self.save_dir.clone();
//         full_path.push(file_path);
//         let str = serde_json::to_string(&item)?;
//         let mut file = fs::File::create(&full_path)?;
//         file.write(str.as_bytes())?;
//         Ok(full_path)
//     }

//     fn build_from_save<'de>(&self, file_path: &SavePathBuf) -> Result<Warrior, Box<dyn Error>> {
//         let mut full_path = self.save_dir.clone();
//         full_path.push(file_path);
//         let str = fs::read_to_string(full_path)?;
//         let w: Warrior = serde_json::from_str(&str)?;
//         Ok(w)
//     }
// }

impl SaveManager<Vec<Warrior>> for WarriorSaveManager {
    fn build(save_dir: SavePathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let warrior_save_manager = Self::new(save_dir);
        if !warrior_save_manager.save_dir.as_path().try_exists()? {
            fs::create_dir(warrior_save_manager.save_dir.as_path())?;
        }
        Ok(warrior_save_manager)
    }

    fn save(&self, item: Vec<Warrior>, file_path: SavePathBuf) -> Result<SavePathBuf, Box<dyn Error>> {
        let mut full_path = self.save_dir.clone();
        full_path.push(file_path);
        let str = serde_json::to_string(&item)?;
        let mut file = fs::File::create(&full_path)?;
        file.write(str.as_bytes())?;
        Ok(full_path)
    }

    fn build_from_save<'de>(&self, file_path: &SavePathBuf) -> Result<Vec<Warrior>, Box<dyn Error>> {
        let mut full_path = self.save_dir.clone();
        full_path.push(file_path);
        let str = fs::read_to_string(full_path)?;
        let w: Vec<Warrior> = serde_json::from_str(&str)?;
        Ok(w)
    }
}
