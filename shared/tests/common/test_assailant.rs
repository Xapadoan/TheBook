use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::assault::assailant::Assailant;
use shared::assault::attack_attempt::{AttackAttempt, AttackThreshold};
use shared::assault::attack_clumsiness::ResolveAttackClumsiness;
use shared::assault::attack_success::ResolveAttackSuccess;
use shared::assault::clumsiness::ResolveClumsiness;
use shared::assault::common_traits::{DealDamages, ReduceDamages, ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults, TakeDamage};
use shared::assault::critical_hit::{DealCriticalHit, ResolveCriticalHit, ResolveCriticalHitSelf};
use shared::assault::critical_parry::{DealCriticalParry, ResolveCriticalParry};
use shared::assault::duration_damages::{DurationDamages, TakeDurationDamages};
use shared::assault::not_possible::{CanAttack, CanBeAttacked};
use shared::assault::parry_attempt::{ParryAttempt, ParryThreshold};
use shared::assault::parry_clumsiness::ResolveParryClumsiness;
use shared::assault::parry_success::ResolveParrySuccess;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::health::{Health, IsDead, IsUnconscious, MutableHealth};
use shared::knock_out::KnockOut;
use shared::name::Name;
use shared::stats::{Stat, StatModifier, Stats, StatsManager};
use shared::temporary_handicap::{
    OptionalAssaultMisses,
    OptionalMutableAssaultMisses,
    OptionalMutableParryMisses,
    OptionalParryMisses,
    TemporaryHandicap,
};
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;

use shared::warrior::body::{Body, HasBody, HasMutableBody};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestAssailant {
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

impl TestAssailant {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::from("Rando"),
            health: Health::new(30, 30),
            weapon: None,
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

impl UniqueEntity for TestAssailant {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Name for TestAssailant {
    fn name(&self) -> &str {
        &self.name
    }
}

impl OptionalMutableWeapon for TestAssailant {
    fn weapon(&self) -> &Option<Weapon> {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut Option<Weapon> {
        &mut self.weapon
    }
}


impl MutableHealth for TestAssailant {
    fn health(&self) -> &Health {
        &self.health
    }

    fn health_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}
impl IsDead for TestAssailant {}
impl IsUnconscious for TestAssailant {
    fn is_unconscious(&self) -> bool {
        self.health.current() < 5 || self.is_unconscious
    }
}

impl TournamentContestant for TestAssailant {
    fn current_tournament(&self) -> &Option<Uuid> {
        &self.current_tournament
    }

    fn set_current_tournament(&mut self, tournament_uuid: Option<Uuid>) {
        self.current_tournament = tournament_uuid
    }
}

impl OptionalAssaultMisses for TestAssailant {
    fn assault_misses(&self) -> &Option<TemporaryHandicap> {
        &self.assault_misses
    }
}

impl OptionalMutableAssaultMisses for TestAssailant {
    fn assault_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.assault_misses
    }
}

impl OptionalParryMisses for TestAssailant {
    fn parry_misses(&self) -> &Option<TemporaryHandicap> {
        &self.parry_misses
    }
}

impl OptionalMutableParryMisses for TestAssailant {
    fn parry_misses_mut(&mut self) -> &mut Option<TemporaryHandicap> {
        &mut self.parry_misses
    }
}

impl HasBody for TestAssailant {
    fn body(&self) -> &Body {
        &self.body
    }
}
impl HasMutableBody for TestAssailant {
    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

impl TakeDurationDamages for TestAssailant {
    fn duration_damages(&self) -> &Vec<DurationDamages> {
        &self.duration_damages
    }
    fn duration_damages_mut(&mut self) -> &mut Vec<DurationDamages> {
        &mut self.duration_damages
    }
}

impl ReduceDamages for TestAssailant {
    fn reduce_damages(&self, damages: u8) -> u8 {
        self.body().reduce_damages(damages)
    }
}

impl DealDamages for TestAssailant {
    fn deal_damages(&self) -> u8 {
        if let Some(weapon) = self.weapon() {
            weapon.deal_damages()
        } else {
            0
        }
    }
}

impl Stats for TestAssailant {
    fn stats(&self) -> &StatsManager {
        &self.stats
    }
}

impl AttackThreshold for TestAssailant {
    fn attack_threshold(&self) -> u8 {
        let mut attack = self.stats().attack().value();
        if let Some(weapon) = self.weapon() {
            attack = weapon.modify_stat(Stat::Attack(attack)).value();
        }
        attack = self.body.modify_stat(Stat::Attack(attack)).value();
        attack
    }
}

impl ParryThreshold for TestAssailant {
    fn parry_threshold(&self) -> u8 {
        let mut parry = self.stats().parry().value();
        if let Some(weapon) = self.weapon() {
            parry = weapon.modify_stat(Stat::Parry(parry)).value();
        }
        parry = self.body.modify_stat(Stat::Parry(parry)).value();
        parry
    }
}

impl KnockOut for TestAssailant {
    fn knock_out(&mut self) {
        self.is_unconscious = true;
    }
}

impl ResolveGougeRandomEye for TestAssailant {}
impl ResolveBreakWeapon for TestAssailant {}
impl ResolveDropWeapon for TestAssailant {}
impl ResolveMissAssaults for TestAssailant {}
impl ResolveAttackSuccess for TestAssailant {}
impl ResolveClumsiness for TestAssailant {}
impl DealCriticalHit for TestAssailant {}
impl ResolveCriticalHit for TestAssailant {}
impl ResolveCriticalHitSelf for TestAssailant {}
impl ResolveAttackClumsiness for TestAssailant {}
impl DealCriticalParry for TestAssailant {}
impl ResolveCriticalParry for TestAssailant {}
impl ResolveParryClumsiness for TestAssailant {}
impl TakeDamage for TestAssailant {}
impl ResolveParrySuccess for TestAssailant {}
impl CanAttack for TestAssailant {}
impl CanBeAttacked for TestAssailant {}
impl AttackAttempt for TestAssailant {}
impl ParryAttempt for TestAssailant {}

impl Assailant for TestAssailant {}
