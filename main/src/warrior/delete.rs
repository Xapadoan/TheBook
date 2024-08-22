use std::path::PathBuf;

use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::{FileRepository, Repository};

use super::WarriorAPIError;

pub fn delete_warrior(uuid: &Uuid) -> Result<(), WarriorAPIError> {
    eprintln!("[WARN] limit possible update fields");
    let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
    repo.delete(uuid)?;
    Ok(())
}
