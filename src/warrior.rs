pub mod body;
pub mod protection;
pub mod stats;

use body::Body;
use body::body_part::BodyPartKind;
use protection::WearProtection;
use stats::StatModifier;
use stats::StatsManager;
use stats::Stat;

use crate::dice::Dice;
use crate::dice::RollResult;
use crate::fight_mechanics::critical_hit::CriticalHitResult;
use crate::fight_mechanics::fight_action::ExecuteFightActionResult;
use crate::fight_mechanics::fight_action::ShowFightActionResult;
use crate::fight_mechanics::assaults_miss::AssaultsMiss;
use crate::fight_mechanics::parries_miss::ParriesMiss;
use crate::fight_mechanics::parry::ParryAttemptResult;
use crate::fight_mechanics::attack::AttackAttemptResult;
use crate::fight_mechanics::ApplyDamageModifier;
use crate::fight_mechanics::CanMissAssaults;
use crate::fight_mechanics::CanMissParries;
use crate::fight_mechanics::CriticalHitOn;
use crate::fight_mechanics::critical_parry::CriticalParry;
use crate::fight_mechanics::IsUnconscious;
use crate::fight_mechanics::TakeReducibleDamage;
use crate::fight_mechanics::{ParryAttempt, AttackAttempt, TemporaryHandicap};
use crate::fight_mechanics::{RollDamage, TakeDamage};
use crate::modifiers::Modifier;
use crate::weapon::CanHaveWeapon;
use crate::weapon::Weapon;

#[derive(Debug)]
pub struct Warrior {
    name: String,
    stats_manager: StatsManager,
    health: u8,
    weapon: Option<Weapon>,
    assaults_miss: Option<AssaultsMiss>,
    parries_miss: Option<ParriesMiss>,
    is_unconscious: bool,
    body: Body,
}

impl Warrior {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            stats_manager: StatsManager::new(),
            health: 30,
            weapon: None,
            assaults_miss: None,
            parries_miss: None,
            is_unconscious: false,
            body: Body::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    pub fn is_dead(&self) -> bool {
        self.health < 1
    }

    pub fn attack(&mut self, target: &mut Self) {
        if !target.has_weapon() {
            return;
        }
        if self.is_dead() || self.is_unconscious() {
            return;
        }
        if self.must_miss_assault() {
            self.miss_assault();
            return;
        }
        println!("{} attacks {}", self.name, target.name);
        let mut attack_attempt_result = self.attack_attempt();
        attack_attempt_result.show_fight_action_result(self, target);
        attack_attempt_result.execute(self, target);
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

impl CanMissParries for Warrior {
    fn must_miss_parry(&self) -> bool {
        self.parries_miss.is_some()
    }

    fn miss_parry(&mut self) {
        let misses = self.parries_miss.as_mut().unwrap();
        misses.decrement_count();
        println!("{} cannot parry because {}", self.name, misses.reason());
        if misses.count() == 0 {
            self.parries_miss = None;
        }
    }

    fn will_miss_parries(&mut self, misses: ParriesMiss) {
        self.parries_miss = Some(misses);
    }
}

impl CanMissAssaults for Warrior {
    fn must_miss_assault(&self) -> bool {
        self.assaults_miss.is_some()
    }

    fn miss_assault(&mut self) {
        let misses = self.assaults_miss.as_mut().unwrap();
        misses.decrement_count();
        println!("{} cannot attack because {}", self.name, misses.reason());
        if misses.count() == 0 {
            self.assaults_miss = None;
        }
    }

    fn will_miss_assault(&mut self, misses: AssaultsMiss) {
        self.assaults_miss = Some(misses)
    }
}

impl AttackAttempt for Warrior {
    fn attack_attempt(&self) -> AttackAttemptResult {
        let success_threshold = self.modify_stat(self.stats_manager.attack_stat());
        match Dice::D6.test_roll(Stat::consume(success_threshold)) {
            RollResult::CriticalSuccess => AttackAttemptResult::CriticalSuccess,
            RollResult::Success => AttackAttemptResult::Success,
            RollResult::Failure => AttackAttemptResult::Failure,
            RollResult::CriticalFailure => AttackAttemptResult::CriticalFailure
        }
    }
}

impl ParryAttempt for Warrior {
    fn parry_attempt(&self) -> ParryAttemptResult {
        if self.weapon.is_none() {
            return ParryAttemptResult::Failure;
        }
        let success_threshold = self.modify_stat(self.stats_manager.parry_stat());
        match Dice::D6.test_roll(Stat::consume(success_threshold)) {
            RollResult::CriticalSuccess => ParryAttemptResult::CriticalSuccess,
            RollResult::Success => ParryAttemptResult::Success,
            RollResult::Failure => ParryAttemptResult::Failure,
            RollResult::CriticalFailure => ParryAttemptResult::CriticalFailure
        }
    }
}

impl CriticalHitOn for Warrior {
    fn critical_hit_on(&self, target: &Warrior) -> CriticalHitResult {
        if self.weapon.is_none() {
            println!("[WARN] bear hands fights is not implemented yet !");
            return CriticalHitResult::roll_blunt(target);
        }

        if self.weapon.as_ref().unwrap().is_sharp() {
            CriticalHitResult::roll_sharp(target)
        } else {
            CriticalHitResult::roll_blunt(target)
        }
    }
}

impl CriticalParry for Warrior {}

impl IsUnconscious for Warrior {
    fn is_unconscious(&self) -> bool {
        self.is_unconscious
    }

    fn set_unconscious(&mut self) {
        println!("{} falls unconscious", self.name);
        self.is_unconscious = true;
    }
}

impl TakeDamage for Warrior {
    fn take_damage(&mut self, dmg: u8) {
        if self.health > dmg {
            self.health -= dmg;
            if self.health < 5 {
                self.set_unconscious();
            }
        } else {
            self.health = 0;
        }
    }
}

impl RollDamage for Warrior {
    fn roll_damage(&self) -> u8 {
        match &self.weapon {
            Some(weapon) => weapon.roll_damage(),
            None => Modifier::new(-2).apply(Dice::D6.roll()),
        }
    }
}

impl ApplyDamageModifier for Warrior {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        self.body.apply_damage_modifier(base)
    }
}

impl TakeReducibleDamage for Warrior {
    fn take_reduced_damage(&mut self, damage: u8) {
        self.take_damage(self.apply_damage_modifier(damage));
    }
}

impl WearProtection for Warrior {
    fn can_wear_protection(&self, protection: &protection::Protection, body_part: BodyPartKind) -> bool {
        self.body.can_wear_protection(protection, body_part)
    }

    fn wear_protection(&mut self, protection: protection::Protection, body_part: BodyPartKind) {
        self.body.wear_protection(protection, body_part)
    }
}

impl StatModifier for Warrior {
    fn modify_stat(&self, base: Stat) -> Stat {
        let mut stat = base;
        stat = match &self.weapon {
            Some(weapon) => weapon.modify_stat(stat),
            None => match stat {
                Stat::Attack(attack) => Stat::Attack(Modifier::new(-4).apply(attack)),
                Stat::Parry(_) => stat,
            }
        };
        stat = self.body.modify_stat(stat);
        stat
    }
}

impl CanHaveWeapon for Warrior {
    fn drop_weapon(&mut self) -> Option<Weapon> {
        if self.weapon.is_none() {
            None
        } else {
            self.weapon.take()
        }
    }

    fn has_weapon(&self) -> bool {
        self.weapon.is_some()
    }

    fn take_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon)
    }

    fn weapon(&self) -> Option<&Weapon> {
        self.weapon.as_ref()
    }

    fn weapon_mut(&mut self) -> Option<&mut Weapon> {
        self.weapon.as_mut()
    }
}
