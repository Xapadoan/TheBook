use chrono::{DateTime, Utc};
use shared::{equipment::weapon::OptionalMutableWeapon, experience::Experience, health::{MutableHealth, PassiveHealing}, name::Name, stats::{StatKind, Stats}, tournament::contestant::TournamentContestant, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateWarriorSchema {
    pub uuid: Uuid,
    pub player_uuid: Uuid,
    pub name: String,
    pub health: u8,
    pub nat_attack: u8,
    pub nat_parry: u8,
    pub nat_courage: u8,
    pub nat_dexterity: u8,
    pub nat_strength: u8,
    pub weapon_uuid: Option<String>,
}
impl CreateWarriorSchema {
    pub fn new(warrior: Warrior, player_uuid: &Uuid, weapon_uuid: &Uuid) -> Self {
        let weapon_uuid_str = Some(weapon_uuid.to_string());
        Self {
            uuid: Uuid::new_v4(),
            player_uuid: player_uuid.clone(),
            name: warrior.name().to_string(),
            health: warrior.health().max(),
            nat_attack: warrior.stats().nat_stat(&StatKind::Attack).value(),
            nat_parry: warrior.stats().nat_stat(&StatKind::Parry).value(),
            nat_courage: warrior.stats().nat_stat(&StatKind::Courage).value(),
            nat_dexterity: warrior.stats().nat_stat(&StatKind::Dexterity).value(),
            nat_strength: warrior.stats().nat_stat(&StatKind::Strength).value(),
            weapon_uuid: weapon_uuid_str,
        }
    }
}

pub struct UpdateWarriorSchema {
    pub current_health: u8,
    pub max_health: u8,
    pub tournament_uuid: Option<String>,
    pub nat_attack: u8,
    pub nat_parry: u8,
    pub nat_courage: u8,
    pub nat_dexterity: u8,
    pub nat_strength: u8,
    pub last_passive_heal: DateTime<Utc>,
    pub experience: u64,
    pub level: u8,
    pub weapon_uuid: Option<String>,
}
impl UpdateWarriorSchema {
    pub fn new(
        current_health: u8,
        max_health: u8,
        tournament_uuid: Option<String>,
        nat_attack: u8,
        nat_parry: u8,
        nat_courage: u8,
        nat_dexterity: u8,
        nat_strength: u8,
        last_passive_heal: DateTime<Utc>,
        experience: u64,
        level: u8,
        weapon_uuid: Option<String>,
    ) -> Self {
        Self {
            current_health,
            max_health,
            tournament_uuid,
            nat_attack,
            nat_parry,
            nat_courage,
            nat_dexterity,
            nat_strength,
            last_passive_heal,
            experience,
            level,
            weapon_uuid,
        }
    }
}
impl From<Warrior> for UpdateWarriorSchema {
    fn from(value: Warrior) -> Self {
        let tournament_uuid = match value.current_tournament() {
            Some(uuid) => Some(uuid.to_string()),
            None => None,
        };
        let weapon_uuid = match value.weapon() {
            Some(weapon) => Some(weapon.uuid().to_string()),
            None => None,
        };
        Self {
            current_health: value.health().current(),
            max_health: value.health().max(),
            tournament_uuid,
            nat_attack: value.stats().nat_stat(&StatKind::Attack).value(),
            nat_parry: value.stats().nat_stat(&StatKind::Parry).value(),
            nat_courage: value.stats().nat_stat(&StatKind::Courage).value(),
            nat_dexterity: value.stats().nat_stat(&StatKind::Dexterity).value(),
            nat_strength: value.stats().nat_stat(&StatKind::Strength).value(),
            last_passive_heal: value.last_passive_heal(),
            experience: value.xp(),
            level: value.level(),
            weapon_uuid,
        }
    }
}