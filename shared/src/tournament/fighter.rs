use uuid::Uuid;

use crate::assault::attack_attempt::{AttackAttempt, AttackThreshold};
use crate::assault::attack_clumsiness::ResolveAttackClumsiness;
use crate::assault::attack_not_possible::{CanAttack, CanBeAttacked};
use crate::assault::attack_success::ResolveAttackSuccess;
use crate::assault::clumsiness::ResolveClumsiness;
use crate::assault::common_traits::{DealDamages, ReduceDamages, ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults, TakeDamage};
use crate::assault::critical_hit::{DealCriticalHit, ResolveCriticalHit, ResolveCriticalHitSelf};
use crate::assault::critical_parry::{DealCriticalParry, ResolveCriticalParry};
use crate::assault::duration_damages::TakeDurationDamages;
use crate::assault::end_turn_consequences::EndTurnConsequencesBuilder;
use crate::assault::parry_attempt::{ParryAttempt, ParryThreshold};
use crate::assault::parry_clumsiness::ResolveParryClumsiness;
use crate::assault::parry_not_possible::CanParry;
use crate::assault::parry_success::ResolveParrySuccess;
use crate::assault::{
    assailant::Assailant,
    assault_order_comparable::AssaultOrderComparable,
    duration_damages::DurationDamages,
};
use crate::equipment::weapon::{OptionalMutableWeapon, Weapon};
use crate::health::{Health, IsDead, IsUnconscious, MutableHealth};
use crate::inventory::{HasInventory, HasMutableInventory, Inventory};
use crate::knock_out::KnockOut;
use crate::name::Name;
use crate::stats::{StatModifier, Stats, StatsManager};
use crate::temporary_handicap::{OptionalAssaultMisses, OptionalMutableAssaultMisses, OptionalMutableParryMisses, OptionalParryMisses, TemporaryHandicap};
use crate::unique_entity::UniqueEntity;
use crate::warrior::body::{Body, HasBody, HasMutableBody};
use crate::warrior::Warrior;

pub struct Fighter {
    uuid: Uuid,
    name: String,
    health: Health,
    weapon: Option<Weapon>,
    // current_tournament: Option<Uuid>,
    assault_misses: Option<TemporaryHandicap>,
    parry_misses: Option<TemporaryHandicap>,
    body: Body,
    duration_damages: Vec<DurationDamages>,
    stats: StatsManager,
    is_unconscious: bool,
    // last_passive_heal: i64,
    // experience: u64,
    inventory: Inventory,
}

impl Fighter {
    pub fn frag(self) -> (
        Uuid,
        Health,
        Option<Weapon>,
        Body,
        Vec<DurationDamages>,
        Inventory,
    ) {
        (
            self.uuid,
            self.health,
            self.weapon,
            self.body,
            self.duration_damages,
            self.inventory,
        )
    }

    pub fn consume(self, warrior: &mut Warrior) -> Inventory {
        *warrior.health_mut() = self.health;
        *warrior.weapon_mut() = self.weapon;
        *warrior.body_mut() = self.body;
        *warrior.duration_damages_mut() = self.duration_damages;
        self.inventory
    }
}

impl From<&Warrior> for Fighter {
    fn from(warrior: &Warrior) -> Self {
        Self {
            uuid: warrior.uuid().clone(),
            name: warrior.name().to_string(),
            health: warrior.health().clone(),
            weapon: warrior.weapon().clone(),
            assault_misses: None,
            parry_misses: None,
            body: warrior.body().clone(),
            duration_damages: warrior.duration_damages().clone(),
            stats: warrior.stats().clone(),
            is_unconscious: warrior.is_unconscious(),
            inventory: Inventory::new(),
        }
    }
}

impl AssaultOrderComparable for Fighter {
    fn assault_order_comparable(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = &self.weapon {
            modifiers.push(Box::new(weapon))
        }
        self.stats.courage(&modifiers).value()
    }
}

impl EndTurnConsequencesBuilder for Fighter {}
impl TakeDurationDamages for Fighter {
    fn duration_damages(&self) -> &Vec<DurationDamages> {
        &self.duration_damages
    }
    fn duration_damages_mut(&mut self) -> &mut Vec<DurationDamages> {
        &mut self.duration_damages
    }
}
impl MutableHealth for Fighter {
    fn health(&self) -> &Health {
        &self.health
    }
    fn health_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}
impl KnockOut for Fighter {
    fn knock_out(&mut self) {
        self.is_unconscious = true;
    }
}
impl AttackThreshold for Fighter {
    fn attack_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = self.weapon() {
            modifiers.push(Box::new(weapon));
        }
        self.stats.attack(&modifiers).value()
    }
}
impl ParryThreshold for Fighter {
    fn parry_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = self.weapon() {
            modifiers.push(Box::new(weapon));
        }
        self.stats.parry(&modifiers).value()
    }
}
impl ReduceDamages for Fighter {
    fn reduce_damages(&self, damages: u8) -> u8 {
        self.body().reduce_damages(damages)
    }
}
impl DealDamages for Fighter {
    fn deal_damages(&self) -> u8 {
        if let Some(weapon) = self.weapon() {
            let mut damages = weapon.deal_damages();
            let str = self.stats.strength(&[Box::new(weapon), Box::new(&self.body)]);
            if str.value() < 8 {
                damages -= 1;
            }
            damages
        } else {
            0
        }
    }
}
impl Stats for Fighter {
    fn stats(&self) -> &StatsManager {
        &self.stats
    }
}
impl OptionalParryMisses for Fighter {
    fn parry_misses(&self) -> &Option<TemporaryHandicap> {
        &self.parry_misses
    }
}
impl OptionalMutableParryMisses for Fighter {
    fn parry_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.parry_misses
    }
}
impl HasBody for Fighter {
    fn body(&self) -> &Body {
        &self.body
    }
}
impl HasMutableBody for Fighter {
    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}
impl OptionalAssaultMisses for Fighter {
    fn assault_misses(&self) -> &Option<TemporaryHandicap> {
        &self.assault_misses
    }
}
impl OptionalMutableAssaultMisses for Fighter {
    fn assault_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.assault_misses
    }
}
impl IsDead for Fighter {}
impl IsUnconscious for Fighter {
    fn is_unconscious(&self) -> bool {
        self.health.current() < 5 || self.is_unconscious
    }
}
impl UniqueEntity for Fighter {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
impl Name for Fighter {
    fn name(&self) -> &str {
        &self.name
    }
}
impl OptionalMutableWeapon for Fighter {
    fn weapon(&self) -> &Option<Weapon> {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut Option<Weapon> {
        &mut self.weapon
    }
}
impl HasInventory for Fighter {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}
impl HasMutableInventory for Fighter {
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

impl ResolveGougeRandomEye for Fighter {}
impl ResolveBreakWeapon for Fighter {}
impl ResolveDropWeapon for Fighter {}
impl ResolveMissAssaults for Fighter {}
impl ResolveAttackSuccess for Fighter {}
impl ResolveClumsiness for Fighter {}
impl DealCriticalHit for Fighter {}
impl ResolveCriticalHit for Fighter {}
impl ResolveCriticalHitSelf for Fighter {}
impl ResolveAttackClumsiness for Fighter {}
impl DealCriticalParry for Fighter {}
impl ResolveCriticalParry for Fighter {}
impl ResolveParryClumsiness for Fighter {}
impl TakeDamage for Fighter {}
impl ResolveParrySuccess for Fighter {}
impl CanAttack for Fighter {}
impl CanBeAttacked for Fighter {}
impl AttackAttempt for Fighter {}
impl ParryAttempt for Fighter {}
impl CanParry for Fighter {}

impl Assailant for Fighter {}