use crate::warrior::Warrior;

pub trait ExecuteFightActionResult {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior);
}
