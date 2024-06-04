use crate::fight_mechanics::{
    ApplyAttackModifier, ApplyParryModifier, CriticalHit, RollDamage, TakeDamage,
};
use crate::{dice::Dice, modifiers::Modifier};

#[derive(Debug)]
pub struct WarHammer {
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl WarHammer {
    pub fn new() -> Self {
        Self {
            dmg_modifier: (Modifier::new(5)),
            attack_modifier: (Modifier::new(-3)),
            parry_modifier: (Modifier::new(-4)),
        }
    }
}

impl<T: TakeDamage> CriticalHit<T> for WarHammer {
    fn critical_hit(&self, target: &mut T) {
        println!("Crit with war hammer")
    }
}

impl RollDamage for WarHammer {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl ApplyAttackModifier for WarHammer {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }
}

impl ApplyParryModifier for WarHammer {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}
