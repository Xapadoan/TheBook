use uuid::Uuid;

use crate::unique_entity::UniqueEntity;

use super::Warrior;

pub trait WarriorCollection {
    fn warriors(&self) -> &Vec<Warrior>;
}

pub trait MutableWarriorCollection: WarriorCollection {
    fn warriors_mut(&mut self) -> &mut Vec<Warrior>;
    fn take_warrior(&mut self, uuid: &Uuid) -> Option<Warrior> {
        let position = self.warriors().iter().position(
            |w|
            {
                w.uuid() == uuid
            }
        );
        match position {
            Some(index) => Some(self.warriors_mut().swap_remove(index)),
            None => None
        }
    }
}
