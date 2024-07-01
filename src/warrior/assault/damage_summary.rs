use crate::warrior::{Name, TakeDamage};
use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::assault::show_action::ShowAction;

pub trait ApplyDamageSummary {
    fn apply_damage_summary<T: TakeDamage>(self, assailant: &mut T, victim: &mut T);
}
#[derive(Debug)]
pub struct DamageSummary {
    to_assailant: u8,
    to_victim: u8,
}

impl DamageSummary {
    pub fn new(to_victim: u8) -> Self {
        Self {
            to_assailant: 0,
            to_victim,
        }
    }

    // pub fn add_damage_to_victim(&mut self, damage: u8) {
    //     self.to_victim += damage;
    // }

    pub fn add_damage_to_assailant(&mut self, damage: u8) {
        self.to_assailant += damage;
    }

    pub fn merge(&mut self, target: DamageSummary, reverse: bool) {
        if reverse {
            self.to_assailant += target.to_victim;
            self.to_victim += target.to_assailant;
        } else {
            self.to_assailant += target.to_assailant;
            self.to_victim += target.to_victim;
        }
    }
}

impl ApplyDamageSummary for DamageSummary {
    fn apply_damage_summary<T: TakeDamage>(self, assailant: &mut T, victim: &mut T) {
        assailant.take_damage(self.to_assailant);
        victim.take_damage(self.to_victim);
    }
}

impl ShowAction for DamageSummary {
    fn show<A, V>(&self, assailant: &A, victim: &V)
        where
            A: MayHaveWeapon + Name + CanMissAssaults,
            V: Name + HasBody
    {
        if self.to_victim > 0 {
            println!("{} took {} points of damage", victim.name(), self.to_victim);
        }
        if self.to_assailant > 0 {
            println!("{} took {} points of damage", assailant.name(), self.to_assailant);
        }
    }
}
