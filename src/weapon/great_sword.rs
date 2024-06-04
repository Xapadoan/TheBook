use crate::fight_mechanics::{
    ApplyAttackModifier, ApplyParryModifier, CriticalHit, RollDamage, TakeDamage,
};
use crate::{dice::Dice, modifiers::Modifier};

#[derive(Debug)]
pub struct GreatSword {
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl GreatSword {
    pub fn new() -> Self {
        Self {
            dmg_modifier: (Modifier::new(5)),
            attack_modifier: (Modifier::new(-3)),
            parry_modifier: (Modifier::new(-4)),
        }
    }
}

impl<T: TakeDamage> CriticalHit<T> for GreatSword {
    fn critical_hit(&self, target: &mut T) {
        println!("Crit with great axe")
    }
}

impl RollDamage for GreatSword {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl ApplyAttackModifier for GreatSword {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }
}

impl ApplyParryModifier for GreatSword {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}
