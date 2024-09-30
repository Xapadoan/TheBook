use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assault::assault_order_comparable::AssaultOrderComparable;
use crate::assault::attack_attempt::{AttackAttempt, AttackThreshold};
use crate::assault::attack_clumsiness::ResolveAttackClumsiness;
use crate::assault::attack_success::ResolveAttackSuccess;
use crate::assault::clumsiness::ResolveClumsiness;
use crate::assault::common_traits::{DealDamages, ReduceDamages, ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults, TakeDamage};
use crate::assault::critical_hit::{DealCriticalHit, ResolveCriticalHit, ResolveCriticalHitSelf};
use crate::assault::critical_parry::{DealCriticalParry, ResolveCriticalParry};
use crate::assault::duration_damages::{DurationDamages, TakeDurationDamages};
use crate::assault::attack_not_possible::CanBeAttacked;
use crate::assault::parry_attempt::{ParryAttempt, ParryThreshold};
use crate::assault::parry_clumsiness::ResolveParryClumsiness;
use crate::assault::parry_success::ResolveParrySuccess;
use crate::assault::end_turn_consequences::EndTurnConsequencesBuilder;
use crate::dice::Dice;
use crate::equipment::weapon::{OptionalMutableWeapon, Weapon};
use crate::experience::{Experience, ExperienceError, ExperienceErrorKind, GainExperience};
use crate::health::{Health, IsDead, IsUnconscious, MutableHealth, MutablePassiveHealing, PassiveHealing};
use crate::knock_out::KnockOut;
use crate::name::Name;
use crate::random::{Random, RandomDictionary};
use crate::stats::{Stat, StatModifier, Stats, StatsManager};
use crate::tournament::contestant::TournamentContestant;
use crate::unique_entity::UniqueEntity;

use super::body::{Body, HasBody, HasMutableBody};
use super::names::WarriorNameDictionary;

#[derive(Debug, Serialize, Deserialize)]
pub struct Warrior {
    uuid: Uuid,
    name: String,
    health: Health,
    weapon: Option<Weapon>,
    current_tournament: Option<Uuid>,
    body: Body,
    duration_damages: Vec<DurationDamages>,
    stats: StatsManager,
    is_unconscious: bool,
    last_passive_heal: i64,
    experience: u64,
    level: u8,
}

impl UniqueEntity for Warrior {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Name for Warrior {
    fn name(&self) -> &str {
        &self.name
    }
}

impl OptionalMutableWeapon for Warrior {
    fn weapon(&self) -> &Option<Weapon> {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut Option<Weapon> {
        &mut self.weapon
    }
}

impl Random for Warrior {
    fn random() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::from(WarriorNameDictionary::random_item()),
            health: Health::new(30, 30),
            weapon: Some(Weapon::random()),
            current_tournament: None,
            body: Body::new(),
            duration_damages: vec![],
            stats: StatsManager::random(),
            is_unconscious: false,
            last_passive_heal: Utc::now().timestamp(),
            experience: 0,
            level: 1,
        }
    }
}

impl MutableHealth for Warrior {
    fn health(&self) -> &Health {
        &self.health
    }

    fn health_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}
impl IsDead for Warrior {}
impl IsUnconscious for Warrior {
    fn is_unconscious(&self) -> bool {
        self.health.current() < 5 || self.is_unconscious
    }
}

impl TournamentContestant for Warrior {
    fn current_tournament(&self) -> &Option<Uuid> {
        &self.current_tournament
    }

    fn set_current_tournament(&mut self, tournament_uuid: Option<Uuid>) {
        self.current_tournament = tournament_uuid
    }
}

impl HasBody for Warrior {
    fn body(&self) -> &Body {
        &self.body
    }
}
impl HasMutableBody for Warrior {
    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

impl TakeDurationDamages for Warrior {
    fn duration_damages(&self) -> &Vec<DurationDamages> {
        &self.duration_damages
    }
    fn duration_damages_mut(&mut self) -> &mut Vec<DurationDamages> {
        &mut self.duration_damages
    }
}

impl ReduceDamages for Warrior {
    fn reduce_damages(&self, damages: u8) -> u8 {
        self.body().reduce_damages(damages)
    }
}

impl DealDamages for Warrior {
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

impl Stats for Warrior {
    fn stats(&self) -> &StatsManager {
        &self.stats
    }
}

impl AttackThreshold for Warrior {
    fn attack_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = self.weapon() {
            modifiers.push(Box::new(weapon));
        }
        self.stats.attack(&modifiers).value()
    }
}

impl ParryThreshold for Warrior {
    fn parry_threshold(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = self.weapon() {
            modifiers.push(Box::new(weapon));
        }
        self.stats.parry(&modifiers).value()
    }
}

impl KnockOut for Warrior {
    fn knock_out(&mut self) {
        self.is_unconscious = true;
    }
}

impl PassiveHealing for Warrior {
    fn last_passive_heal(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.last_passive_heal, 0).unwrap()
    }
}

// server only
impl MutablePassiveHealing for Warrior {
    fn set_last_passive_heal(&mut self, last_passive_heal: DateTime<Utc>) {
        self.last_passive_heal = last_passive_heal.timestamp()
    }
}

impl Experience for Warrior {
    fn xp(&self) -> u64 {
        self.experience
    }
    fn level(&self) -> u8 {
        self.level
    }
}
impl GainExperience for Warrior {
    fn gain_xp(&mut self, xp: u64) {
        self.experience += xp;
    }
    fn level_up(&mut self, stat: &Stat) -> Result<(), ExperienceError> {
        let stat_is_incrementable = if (self.level + 1) % 2 == 0 {
            match stat {
                Stat::Courage(_) | Stat::Dexterity(_) | Stat::Strength(_) => true,
                _ => false,
            }
        } else {
            match stat {
                Stat::Attack(_) | Stat::Parry(_) => true,
                _ => false,
            }
        };
        if !stat_is_incrementable {
            return Err(ExperienceError::new(
                &ExperienceErrorKind::InvalidStatIncrement(self.level + 1, stat.clone()),
            ));
        }
        let health_gain = Dice::D6.roll();
        let current = self.health.current();
        let max = self.health.max();
        self.health.set_max(max + health_gain);
        self.health.set(current + health_gain);
        self.stats.increment_nat_stat(stat);
        self.level += 1;
        Ok(())
    }
}

impl AssaultOrderComparable for Warrior {
    fn assault_order_comparable(&self) -> u8 {
        let mut modifiers: Vec<Box<&dyn StatModifier>> = vec![Box::new(&self.body)];
        if let Some(weapon) = &self.weapon {
            modifiers.push(Box::new(weapon))
        }
        self.stats.courage(&modifiers).value()
    }
}

impl EndTurnConsequencesBuilder for Warrior {}

impl ResolveGougeRandomEye for Warrior {}
impl ResolveBreakWeapon for Warrior {}
impl ResolveDropWeapon for Warrior {}
impl ResolveMissAssaults for Warrior {}
impl ResolveAttackSuccess for Warrior {}
impl ResolveClumsiness for Warrior {}
impl DealCriticalHit for Warrior {}
impl ResolveCriticalHit for Warrior {}
impl ResolveCriticalHitSelf for Warrior {}
impl ResolveAttackClumsiness for Warrior {}
impl DealCriticalParry for Warrior {}
impl ResolveCriticalParry for Warrior {}
impl ResolveParryClumsiness for Warrior {}
impl TakeDamage for Warrior {}
impl ResolveParrySuccess for Warrior {}
impl CanBeAttacked for Warrior {}
impl AttackAttempt for Warrior {}
impl ParryAttempt for Warrior {}
