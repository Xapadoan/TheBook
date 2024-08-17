use crate::end_turn_consequences::EndTurnConsequencesBuilder;
use crate::health::IsDead;
use crate::health::IsUnconscious;
use crate::knock_out::KnockOut;
use crate::temporary_handicap::OptionalMutableAssaultMisses;
use crate::temporary_handicap::OptionalMutableParryMisses;
use crate::unique_entity::UniqueEntity;
use crate::warrior::body::HasMutableBody;

use super::attack_attempt::AttackAttempt;
use super::attack_clumsiness::ResolveAttackClumsiness;
use super::attack_success::ResolveAttackSuccess;
use super::common_traits::TakeDamage;
use super::critical_hit::DealCriticalHit;
use super::critical_hit::ResolveCriticalHit;
use super::critical_parry::DealCriticalParry;
use super::critical_parry::ResolveCriticalParry;
use super::duration_damages::TakeDurationDamages;
use super::common_traits::ReduceDamages;
use super::common_traits::DealDamages;
use super::not_possible::CanAttack;
use super::not_possible::CanBeAttacked;
use super::parry_attempt::ParryAttempt;
use super::parry_clumsiness::ResolveParryClumsiness;
use super::parry_not_possible::CanParry;
use super::parry_success::ResolveParrySuccess;

pub trait Assailant:
    UniqueEntity +
    DealCriticalHit +
    ResolveCriticalHit +
    ResolveAttackClumsiness +
    ResolveAttackSuccess +
    DealCriticalParry +
    ResolveCriticalParry +
    ResolveParryClumsiness +
    ResolveParrySuccess +
    DealDamages +
    ReduceDamages +
    TakeDamage +
    TakeDurationDamages +
    IsDead +
    IsUnconscious +
    OptionalMutableAssaultMisses +
    OptionalMutableParryMisses +
    HasMutableBody +
    CanAttack +
    CanBeAttacked +
    CanParry +
    AttackAttempt +
    ParryAttempt +
    KnockOut +
    EndTurnConsequencesBuilder
{}
