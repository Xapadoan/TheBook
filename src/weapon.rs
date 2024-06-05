use crate::fight_mechanics::critical_hit_option::{roll_blunt_critical, roll_sharp_critical, CriticalHitConsequence};
use crate::fight_mechanics::{
    ApplyAttackModifier, ApplyParryModifier, CriticalHit,
    RollDamage,
};
use crate::modifiers::Modifier;
use crate::dice::Dice;

pub enum WeaponKind {
    Sword,
    GreatSword,
    Axe,
    BattleAxe,
    Hammer,
    WarHammer,
}

#[derive(Debug)]
pub struct Weapon {
    is_sharp: bool,
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl Weapon {
    pub fn new(kind: WeaponKind) -> Self {
        match kind {
            WeaponKind::Sword => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-1),
            },
            WeaponKind::Axe => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
            },
            WeaponKind::BattleAxe => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
            WeaponKind::GreatSword => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
            WeaponKind::Hammer => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
            },
            WeaponKind::WarHammer => Self {
                is_sharp: true,
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
        }
    }
}

impl CriticalHit for Weapon {
    fn critical_hit(&self) -> CriticalHitConsequence {
        if self.is_sharp {
            return roll_sharp_critical()
        } else {
            return roll_blunt_critical()
        }
    }
}

impl RollDamage for Weapon {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl ApplyAttackModifier for Weapon {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }
}

impl ApplyParryModifier for Weapon {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}