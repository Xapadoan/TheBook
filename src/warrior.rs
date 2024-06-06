use crate::dice::Dice;
use crate::dice::RollResult;
use crate::fight_mechanics::CriticalHit;
use crate::fight_mechanics::IsAlive;
use crate::fight_mechanics::{ApplyAttackModifier, ApplyParryModifier, RollDamage, TakeDamage};
use crate::weapon::Weapon;

#[derive(Debug)]
pub struct Warrior {
    name: String,
    health: u8,
    attack: u8,
    parry: u8,
    weapon: Weapon,
}

impl Warrior {
    pub fn new(name: &str, weapon: Weapon) -> Self {
        Self {
            name: String::from(name),
            health: 30,
            attack: 8,
            parry: 10,
            weapon,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    // Fast exit make code more readable ?
    pub fn attack(&mut self, target: &mut Self) {
        println!("{} attacks {}", self.name, target.name);
        match self.attack_test() {
            RollResult::CriticalFailure => println!("{} missed miserably", self.name),
            RollResult::Failure => println!("{} missed", self.name),
            RollResult::Success => match target.parry_test() {
                RollResult::CriticalSuccess => {
                    self.take_damage(self.weapon.roll_damage());
                    println!("{} parried perfectly", target.name);
                }
                RollResult::Success => println!("{} parried", target.name),
                RollResult::Failure => {
                    target.take_damage(self.weapon.roll_damage());
                    println!("{} was hit", target.name)
                }
                RollResult::CriticalFailure => {
                    target.take_damage(self.weapon.roll_damage() * 3);
                    println!("{} failed to parry miserably", target.name)
                }
            },
            RollResult::CriticalSuccess => {
                let crit_consequence = self.weapon.critical_hit();
                let damage = crit_consequence.modify_damages(self.weapon.roll_damage());
                target.take_damage(damage);
                crit_consequence.show(&self, target)
            }
        }
    }

    fn attack_test(&self) -> RollResult {
        let success_threshold = self.weapon.apply_attack_modifier(self.attack);
        Dice::D20.test_roll(success_threshold)
    }

    fn parry_test(&self) -> RollResult {
        let success_threshold = self.weapon.apply_parry_modifier(self.parry);
        Dice::D6.test_roll(success_threshold)
    }
}

impl IsAlive for Warrior {
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

impl TakeDamage for Warrior {
    fn take_damage(&mut self, dmg: u8) {
        if self.health > dmg {
            self.health -= dmg;
        } else {
            self.health = 0;
        }
    }
}
