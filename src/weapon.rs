use crate::fight_mechanics::{
    critical_hit_option::CriticalConsequence, ApplyAttackModifier, ApplyParryModifier, CriticalHit,
    RollDamage,
};
use axe::Axe;
use battle_axe::BattleAxe;
use great_sword::GreatSword;
use hammer::Hammer;
use sword::Sword;
use war_hammer::WarHammer;

pub mod axe;
pub mod battle_axe;
pub mod great_sword;
pub mod hammer;
pub mod sword;
pub mod war_hammer;

pub enum WeaponKind {
    Sword,
    GreatSword,
    Axe,
    BattleAxe,
    Hammer,
    WarHammer,
}

#[derive(Debug)]
pub enum Weapon {
    Sword(Sword),
    Hammer(Hammer),
    Axe(Axe),
    BattleAxe(BattleAxe),
    GreatSword(GreatSword),
    WarHammer(WarHammer),
}

impl RollDamage for Weapon {
    fn roll_damage(&self) -> u8 {
        match self {
            Weapon::Sword(sword) => sword.roll_damage(),
            Weapon::Hammer(hammer) => hammer.roll_damage(),
            Weapon::Axe(axe) => axe.roll_damage(),
            Weapon::BattleAxe(battle_axe) => battle_axe.roll_damage(),
            Weapon::GreatSword(great_sword) => great_sword.roll_damage(),
            Weapon::WarHammer(war_hammer) => war_hammer.roll_damage(),
        }
    }
}

impl ApplyAttackModifier for Weapon {
    fn apply_attack_modifier(&self, base: u8) -> u8 {
        match self {
            Weapon::Sword(sword) => sword.apply_attack_modifier(base),
            Weapon::Hammer(hammer) => hammer.apply_attack_modifier(base),
            Weapon::Axe(axe) => axe.apply_attack_modifier(base),
            Weapon::BattleAxe(battle_axe) => battle_axe.apply_attack_modifier(base),
            Weapon::GreatSword(great_sword) => great_sword.apply_attack_modifier(base),
            Weapon::WarHammer(war_hammer) => war_hammer.apply_attack_modifier(base),
        }
    }
}

impl ApplyParryModifier for Weapon {
    fn apply_parry_modifier(&self, base: u8) -> u8 {
        match self {
            Weapon::Sword(sword) => sword.apply_parry_modifier(base),
            Weapon::Hammer(hammer) => hammer.apply_parry_modifier(base),
            Weapon::Axe(axe) => axe.apply_parry_modifier(base),
            Weapon::BattleAxe(battle_axe) => battle_axe.apply_parry_modifier(base),
            Weapon::GreatSword(great_sword) => great_sword.apply_parry_modifier(base),
            Weapon::WarHammer(war_hammer) => war_hammer.apply_parry_modifier(base),
        }
    }
}

impl CriticalHit for Weapon {
    fn critical_hit(&self) -> CriticalConsequence {
        match self {
            Weapon::Sword(sword) => sword.critical_hit(),
            Weapon::Hammer(hammer) => hammer.critical_hit(),
            Weapon::Axe(sword) => sword.critical_hit(),
            Weapon::BattleAxe(sword) => sword.critical_hit(),
            Weapon::GreatSword(sword) => sword.critical_hit(),
            Weapon::WarHammer(sword) => sword.critical_hit(),
        }
    }
}
