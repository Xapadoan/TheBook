use crate::fight_mechanics::critical_hit_option::{roll_sharp_critical, CriticalConsequence};
use crate::fight_mechanics::{ApplyAttackModifier, ApplyParryModifier, CriticalHit, RollDamage};
use crate::{dice::Dice, modifiers::Modifier};

#[derive(Debug)]
pub struct BattleAxe {
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl BattleAxe {
    pub fn new() -> Self {
        Self {
            dmg_modifier: (Modifier::new(5)),
            attack_modifier: (Modifier::new(-3)),
            parry_modifier: (Modifier::new(-4)),
        }
    }
}

impl CriticalHit for BattleAxe {
    fn critical_hit(&self) -> CriticalConsequence {
        roll_sharp_critical()
    }
}

impl RollDamage for BattleAxe {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl ApplyAttackModifier for BattleAxe {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }
}

impl ApplyParryModifier for BattleAxe {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}
