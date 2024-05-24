use crate::dice::Dice;
use crate::modifiers::Modifier;

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
    dmg_modifier: Modifier,
    attack_modifier: Modifier,
    parry_modifier: Modifier,
}

impl Weapon {
    pub fn new(kind: WeaponKind) -> Weapon {
        match kind {
            WeaponKind::Sword => Weapon {
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-1),
            },
            WeaponKind::GreatSword => Weapon {
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
            WeaponKind::Axe => Weapon {
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
            },
            WeaponKind::BattleAxe => Weapon {
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
            WeaponKind::Hammer => Weapon {
                dmg_modifier: Modifier::new(3),
                attack_modifier: Modifier::new(0),
                parry_modifier: Modifier::new(-2),
            },
            WeaponKind::WarHammer => Weapon {
                dmg_modifier: Modifier::new(5),
                attack_modifier: Modifier::new(-3),
                parry_modifier: Modifier::new(-4),
            },
        }
    }

    pub fn roll_damage(&self) -> u8 {
        self.dmg_modifier.apply(Dice::D6.roll())
    }

    pub fn apply_attack_modifier(&self, base: u8) -> u8 {
        self.attack_modifier.apply(base)
    }

    pub fn apply_parry_modifier(&self, base: u8) -> u8 {
        self.parry_modifier.apply(base)
    }
}
