use crate::fight_mechanics::{
    ApplyAttackModifier, ApplyParryModifier, CriticalHit, RollDamage, TakeDamage,
};
use crate::{dice::Dice, modifiers::Modifier};

#[derive(Debug)]
pub struct Hammer {
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl Hammer {
    pub fn new() -> Self {
        Self {
            dmg_modifier: (Modifier::new(3)),
            attack_modifier: (Modifier::new(0)),
            parry_modifier: (Modifier::new(-2)),
        }
    }
}

impl<T: TakeDamage> CriticalHit<T> for Hammer {
    fn critical_hit(&self, target: &mut T) {
        println!("Crit with hammer")
    }
}

impl RollDamage for Hammer {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl ApplyAttackModifier for Hammer {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }
}

impl ApplyParryModifier for Hammer {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}
