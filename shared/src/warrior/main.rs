use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assault::assailant::Assailant;
use crate::assault::attack_attempt::{AttackAttempt, AttackThreshold};
use crate::assault::attack_clumsiness::ResolveAttackClumsiness;
use crate::assault::attack_success::ResolveAttackSuccess;
use crate::assault::clumsiness::ResolveClumsiness;
use crate::assault::common_traits::{DealDamages, ReduceDamages, ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults, TakeDamage};
use crate::assault::critical_hit::{DealCriticalHit, ResolveCriticalHit, ResolveCriticalHitSelf};
use crate::assault::critical_parry::{DealCriticalParry, ResolveCriticalParry};
use crate::assault::duration_damages::{DurationDamages, TakeDurationDamages};
use crate::assault::not_possible::{CanAttack, CanBeAttacked};
use crate::assault::parry_attempt::{ParryAttempt, ParryThreshold};
use crate::assault::parry_clumsiness::ResolveParryClumsiness;
use crate::assault::parry_not_possible::CanParry;
use crate::assault::parry_success::ResolveParrySuccess;
use crate::assault::end_turn_consequences::EndTurnConsequencesBuilder;
use crate::equipment::weapon::{OptionalMutableWeapon, Weapon};
use crate::health::{Health, IsDead, IsUnconscious, MutableHealth};
use crate::knock_out::KnockOut;
use crate::name::Name;
use crate::random::{Random, RandomDictionary};
use crate::stats::{Stat, StatModifier, Stats, StatsManager};
use crate::temporary_handicap::{
    OptionalAssaultMisses,
    OptionalMutableAssaultMisses,
    OptionalMutableParryMisses,
    OptionalParryMisses,
    TemporaryHandicap,
};
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
    assault_misses: Option<TemporaryHandicap>,
    parry_misses: Option<TemporaryHandicap>,
    body: Body,
    duration_damages: Vec<DurationDamages>,
    stats: StatsManager,
    is_unconscious: bool,
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
            assault_misses: None,
            parry_misses: None,
            body: Body::new(),
            duration_damages: vec![],
            stats: StatsManager::new(),
            is_unconscious: false,
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

impl OptionalAssaultMisses for Warrior {
    fn assault_misses(&self) -> &Option<TemporaryHandicap> {
        &self.assault_misses
    }
}

impl OptionalMutableAssaultMisses for Warrior {
    fn assault_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.assault_misses
    }
}

impl OptionalParryMisses for Warrior {
    fn parry_misses(&self) -> &Option<TemporaryHandicap> {
        &self.parry_misses
    }
}

impl OptionalMutableParryMisses for Warrior {
    fn parry_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.parry_misses
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
            weapon.deal_damages()
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
        let mut attack = self.stats().attack().value();
        if let Some(weapon) = self.weapon() {
            attack = weapon.modify_stat(Stat::Attack(attack)).value();
        }
        attack = self.body.modify_stat(Stat::Attack(attack)).value();
        attack
    }
}

impl ParryThreshold for Warrior {
    fn parry_threshold(&self) -> u8 {
        let mut parry = self.stats().parry().value();
        if let Some(weapon) = self.weapon() {
            parry = weapon.modify_stat(Stat::Parry(parry)).value();
        }
        parry = self.body.modify_stat(Stat::Parry(parry)).value();
        parry
    }
}

impl KnockOut for Warrior {
    fn knock_out(&mut self) {
        self.is_unconscious = true;
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
impl CanAttack for Warrior {}
impl CanBeAttacked for Warrior {}
impl AttackAttempt for Warrior {}
impl ParryAttempt for Warrior {}
impl CanParry for Warrior {}

impl Assailant for Warrior {}
