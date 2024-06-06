use crate::warrior::Warrior;

pub trait ShowFightActionResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior);
}

pub trait ApplyFightActionResult {
    fn apply_fight_action_result(&self, assailant: &mut Warrior, victim: &mut Warrior);
}