use crate::warrior::Warrior;

use super::damage_summary::DamageSummary;

pub trait ExecuteAction {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) -> DamageSummary;
}
