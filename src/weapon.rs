use crate::fight_mechanics::RollDamage;
use crate::modifiers::Modifier;
use crate::dice::Dice;
use crate::warrior::stats::{Stat, StatModifier};

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

    pub fn is_sharp(&self) -> bool {
        self.is_sharp
    }
}

impl RollDamage for Weapon {
    fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }
}

impl StatModifier for Weapon {
    fn modify_stat(&self, base: Stat) -> Stat {
        match base {
            Stat::Attack(attack) => Stat::Attack(self.attack_modifier.apply(attack)),
            Stat::Parry(parry) => Stat::Parry(self.parry_modifier.apply(parry))
        }
    }
}
