use crate::health::MutableHealth;

pub trait DealDamages {
    fn deal_damages(&self) -> u8;
}

pub trait TakeDamage: MutableHealth {
    fn take_damage(&mut self, damages: u8) {
        let current_health = self.health().current();
        let new_health = if current_health < damages {
            0
        } else {
            current_health - damages
        };
        self.health_mut().set(new_health)
    }
}

pub trait ReduceDamages {
    fn reduce_damages(&self, damages: u8) -> u8;
}

pub trait AddDamages {
    fn add_damages(&self, damages: u8) -> u8;
}

pub trait TakeReducedDamage: TakeDamage + ReduceDamages {
    fn take_reduced_damages(&mut self, damages: u8) {
        self.take_damage(self.reduce_damages(damages));
    }
}

