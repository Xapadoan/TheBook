use serde::{Deserialize, Serialize};

use crate::equipment::protection::OptionalMutableProtection;
use crate::equipment::rupture::{Rupture, RUPTURE_MAX};
use crate::inventory::{Item, MutableItems};
use crate::warrior::body::body_part::{BodyPartKind, OptionalMutableBodyPart};
use crate::warrior::body::injury::{Injuries, Injury};
use crate::temporary_handicap::TemporaryHandicap;

use super::assailant::Assailant;
use super::critical_hit::CriticalHit;
use super::duration_damages::DurationDamages;

pub trait AssaultConsequencesBuilder {
    fn to_consequences(&self, assailant: &dyn Assailant, victim: & dyn Assailant) -> AssaultConsequences;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssaultConsequences {
    for_assailant: IndividualConsequences,
    for_victim: IndividualConsequences,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndividualConsequences {
    damages: u8,
    raw_damages: u8,
    armor_damages: Option<ArmorDamages>,
    injury: Option<Injury>,
    duration_damages: Option<DurationDamages>,
    knock_out: bool,
    assault_misses: Option<TemporaryHandicap>,
    parry_misses: Option<TemporaryHandicap>,
    drop_weapon: bool,
    weapon_damages: Option<u8>,
    counter_critical_hit: Option<CriticalHit>,
    self_critical_hit: Option<CriticalHit>,
}

impl IndividualConsequences {
    pub fn new(
        damages: u8,
        raw_damages: u8,
        armor_damages: Option<ArmorDamages>,
        injury: Option<Injury>,
        duration_damages: Option<DurationDamages>,
        knock_out: bool,
        assault_misses: Option<TemporaryHandicap>,
        parry_misses: Option<TemporaryHandicap>,
        drop_weapon: bool,
        weapon_damages: Option<u8>,
        counter_critical_hit: Option<CriticalHit>,
        self_critical_hit: Option<CriticalHit>,
    ) -> Self {
        Self {
            damages,
            raw_damages,
            armor_damages,
            injury,
            duration_damages,
            knock_out,
            assault_misses,
            parry_misses,
            drop_weapon,
            weapon_damages,
            counter_critical_hit,
            self_critical_hit,
        }
    }

    pub fn no_consequences() -> Self {
        Self {
            damages: 0,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn only_damages(damages: u8) -> Self {
        Self {
            damages,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn only_raw_damages(raw_damages: u8) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn damage_armor(raw_damages: u8, armor_damages: ArmorDamages) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: Some(armor_damages),
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn injures(
        raw_damages: u8,
        injury: Injury,
    ) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: None,
            injury: Some(injury),
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn injures_and_damages_armor(
        raw_damages: u8,
        injury: Injury,
        armor_damages: ArmorDamages,
    ) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: Some(armor_damages),
            injury: Some(injury),
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn damage_on_duration(raw_damages: u8, duration_damages: DurationDamages) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: None,
            injury: None,
            duration_damages: Some(duration_damages),
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn knock_out(raw_damages: u8) -> Self {
        Self {
            damages: 0,
            raw_damages,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn miss_assaults(misses: TemporaryHandicap) -> Self {
        Self {
            damages: 0,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: Some(misses),
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn unstoppable_assaults(misses: TemporaryHandicap) -> Self {
        Self {
            damages: 0,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: Some(misses.clone()),
            parry_misses: Some(misses),
            drop_weapon: false,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn drop_weapon() -> Self {
        Self {
            damages: 0,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: true,
            weapon_damages: None,
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    pub fn damage_weapon(rupture_damages: u8) -> Self {
        Self {
            damages: 0,
            raw_damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            parry_misses: None,
            drop_weapon: false,
            weapon_damages: Some(rupture_damages),
            counter_critical_hit: None,
            self_critical_hit: None,
        }
    }

    fn apply(&self, victim: &mut dyn Assailant) {
        victim.take_damage(self.damages);
        victim.take_damage(self.raw_damages);
        if let Some(armor_damages) = &self.armor_damages {
            armor_damages.apply(victim);
        }
        if let Some(injury) = &self.injury {
            match &injury {
                Injury::RightArmSevered |
                Injury::RightHandSevered => {
                    if let Some(weapon) = victim.weapon_mut().take() {
                        victim.inventory_mut().add_item(Item::Weapon(weapon));
                    }
                },
                Injury::LeftArmSevered |
                Injury::LeftHandSevered => {
                    if let Some(weapon) = victim.weapon() {
                        if weapon.is_two_handed() {
                            let lost_weapon = victim.weapon_mut().take().unwrap();
                            victim.inventory_mut().add_item(Item::Weapon(lost_weapon));
                        }
                    }
                }
                _ => {},
            }
            let severed_parts = victim.body_mut().add_injury(injury.clone());
            for mut part in severed_parts.into_iter() {
                if let Some(protection) = part.protection_mut().take() {
                    victim.inventory_mut().add_item(Item::Protection(protection));
                }
            }
        }
        if self.knock_out {
            victim.knock_out();
        }
        if let Some(misses) = &self.assault_misses {
            victim.assault_misses_mut().replace(misses.clone());
        }
        if let Some(misses) = &self.parry_misses {
            victim.parry_misses_mut().replace(misses.clone());
            victim.assault_misses_mut().replace(misses.clone());
        }
        if self.drop_weapon {
            if let Some(weapon) = victim.weapon_mut().take() {
                victim.inventory_mut().add_item(Item::Weapon(weapon));
            }
        }
        if let Some(rupture_damages) = &self.weapon_damages {
            if let Some(weapon) = victim.weapon_mut() {
                weapon.damage_rupture(*rupture_damages);
                if let Some(rup) = weapon.rupture() {
                    if *rup >= RUPTURE_MAX {
                        victim.weapon_mut().take();
                    }
                }
            }
        }
    }

    pub fn weapon_damages(&self) -> &Option<u8> {
        &self.weapon_damages
    }
    pub fn injury(&self) -> &Option<Injury> {
        &self.injury
    }
    pub fn armor_damages(&self) -> &Option<ArmorDamages> {
        &self.armor_damages
    }
    pub fn duration_damages(&self) -> &Option<DurationDamages> {
        &self.duration_damages
    }
    pub fn damages(&self) -> u8 {
        self.damages
    }
    pub fn raw_damages(&self) -> u8 {
        self.raw_damages
    }
    pub fn counter_critical_hit(&self) -> &Option<CriticalHit> {
        &self.counter_critical_hit
    }
    pub fn self_critical_hit(&self) -> &Option<CriticalHit> {
        &self.self_critical_hit
    }
    pub fn add_counter_critical_hit(&mut self, critical_hit: CriticalHit) {
        self.counter_critical_hit = Some(critical_hit)
    }
    pub fn add_self_critical_hit(&mut self, critical_hit: CriticalHit) {
        self.self_critical_hit = Some(critical_hit)
    }
}

impl AssaultConsequences {
    pub fn new(for_assailant: IndividualConsequences, for_victim: IndividualConsequences) -> Self {
        Self {
            for_assailant,
            for_victim,
        }
    }

    pub fn reversed(original: &Self) -> Self {
        Self {
            for_assailant: original.for_victim.clone(),
            for_victim: original.for_assailant.clone(),
        }
    }

    pub fn apply(&self,
        assailant: &mut dyn Assailant,
        victim: &mut dyn Assailant,
    ) {
        self.for_assailant.apply(assailant);
        self.for_victim.apply(victim);
        if assailant.assault_misses().is_some() && self.for_assailant.assault_misses.is_none() {
            assailant.miss_assault();
        }
        if victim.parry_misses().is_some() && self.for_victim.parry_misses.is_none() {
            victim.miss_parry();
        }
    }

    pub fn for_assailant(&self) -> &IndividualConsequences {
        &self.for_assailant
    }

    pub fn for_victim(&self) -> &IndividualConsequences {
        &self.for_victim
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmorDamages {
    damages: u8,
    body_part_kind: BodyPartKind,
}

impl ArmorDamages {
    pub fn new(damages: u8, body_part_kind: BodyPartKind) -> Self {
        Self { damages, body_part_kind }
    }

    pub fn apply(&self, victim: &mut dyn Assailant) {
        let body_part = victim.body_mut().body_part_mut(&self.body_part_kind).as_mut().unwrap();
        let protection = body_part.protection_mut().as_mut().unwrap();
        protection.damage_rupture(self.damages);
        if let Some(rupture) = protection.rupture() {
            if !(*rupture < RUPTURE_MAX) {
                body_part.protection_mut().take();
            }
        }
    }

    pub fn body_part_kind(&self) -> &BodyPartKind {
        &self.body_part_kind
    }

    pub fn damages(&self) -> u8 {
        self.damages
    }
}

#[cfg(test)]
mod tests {
    use crate::temporary_handicap::TemporaryHandicapReason;

    use super::*;

    #[test]
    fn test_individual_consequence_miss_assaults() {
        let actual = IndividualConsequences::miss_assaults(
            TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
        );
        assert_eq!(actual.damages, 0);
        assert_eq!(actual.armor_damages.is_none(), true);
        assert_eq!(actual.injury, None);
        assert_eq!(actual.duration_damages.is_none(), true);
        assert_eq!(actual.knock_out, false);
        assert_eq!(actual.assault_misses.is_some(), true);
        assert_eq!(actual.assault_misses.unwrap().count(), 2);
        assert_eq!(actual.parry_misses.is_none(), true);
        assert_eq!(actual.drop_weapon, false);
        assert_eq!(actual.weapon_damages.is_none(), true);
        assert_eq!(actual.counter_critical_hit.is_none(), true);
        assert_eq!(actual.self_critical_hit.is_none(), true);
    }

    #[test]
    fn test_individual_consequence_unstoppable_assaults() {
        let actual = IndividualConsequences::unstoppable_assaults(
            TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown),
        );
        assert_eq!(actual.damages, 0);
        assert_eq!(actual.armor_damages.is_none(), true);
        assert_eq!(actual.injury, None);
        assert_eq!(actual.duration_damages.is_none(), true);
        assert_eq!(actual.knock_out, false);
        assert_eq!(actual.assault_misses.is_some(), true);
        assert_eq!(actual.parry_misses.is_some(), true);
        assert_eq!(actual.drop_weapon, false);
        assert_eq!(actual.weapon_damages.is_none(), true);
        assert_eq!(actual.counter_critical_hit.is_none(), true);
        assert_eq!(actual.self_critical_hit.is_none(), true);
    }

    #[test]
    fn test_individual_consequence_drop_weapon() {
        let actual = IndividualConsequences::drop_weapon();
        assert_eq!(actual.damages, 0);
        assert_eq!(actual.armor_damages.is_none(), true);
        assert_eq!(actual.injury, None);
        assert_eq!(actual.duration_damages.is_none(), true);
        assert_eq!(actual.knock_out, false);
        assert_eq!(actual.assault_misses.is_none(), true);
        assert_eq!(actual.parry_misses.is_none(), true);
        assert_eq!(actual.drop_weapon, true);
        assert_eq!(actual.weapon_damages.is_none(), true);
        assert_eq!(actual.counter_critical_hit.is_none(), true);
        assert_eq!(actual.self_critical_hit.is_none(), true);
    }

    #[test]
    fn test_individual_consequence_damage_weapon() {
        let actual = IndividualConsequences::damage_weapon(2);
        assert_eq!(actual.damages, 0);
        assert_eq!(actual.armor_damages.is_none(), true);
        assert_eq!(actual.injury, None);
        assert_eq!(actual.duration_damages.is_none(), true);
        assert_eq!(actual.knock_out, false);
        assert_eq!(actual.assault_misses.is_none(), true);
        assert_eq!(actual.parry_misses.is_none(), true);
        assert_eq!(actual.drop_weapon, false);
        assert_eq!(actual.weapon_damages.is_some(), true);
        assert_eq!(actual.weapon_damages.unwrap(), 2);
        assert_eq!(actual.counter_critical_hit.is_none(), true);
        assert_eq!(actual.self_critical_hit.is_none(), true);
    }
}
