use serde::{Deserialize, Serialize};

use crate::equipment::protection::OptionalMutableProtection;
use crate::equipment::rupture::{Rupture, RUPTURE_MAX};
use crate::warrior::body::body_part::{BodyPartKind, OptionalMutableBodyPart};
use crate::warrior::body::injury::Injury;
use crate::temporary_handicap::TemporaryHandicap;

use super::assailant::Assailant;
use super::duration_damages::DurationDamages;

pub trait AssaultConsequencesBuilder {
    fn to_consequences(&self, assailant: &dyn Assailant, victim: & dyn Assailant) -> AssaultConsequences;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssaultConsequences {
    for_assailant: IndividualConsequences,
    for_victim: IndividualConsequences,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualConsequences {
    damages: u8,
    armor_damages: Option<ArmorDamages>,
    injury: Option<Injury>,
    duration_damages: Option<DurationDamages>,
    knock_out: bool,
    assault_misses: Option<TemporaryHandicap>,
    // Continue on this
    unstoppable_assaults: Option<u8>,
    drop_weapon: bool,
    weapon_damages: Option<u8>,
}

impl IndividualConsequences {
    pub fn new(
        damages: u8,
        armor_damages: Option<ArmorDamages>,
        injury: Option<Injury>,
        duration_damages: Option<DurationDamages>,
        knock_out: bool,
        assault_misses: Option<TemporaryHandicap>,
        unstoppable_assaults: Option<u8>,
        drop_weapon: bool,
        weapon_damages: Option<u8>,
    ) -> Self {
        Self {
            damages,
            armor_damages,
            injury,
            duration_damages,
            knock_out,
            assault_misses,
            unstoppable_assaults,
            drop_weapon,
            weapon_damages,
        }
    }

    pub fn no_consequences() -> Self {
        Self {
            damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn only_damages(damages: u8) -> Self {
        Self {
            damages,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn damage_armor(damages: u8, armor_damages: ArmorDamages) -> Self {
        Self {
            damages,
            armor_damages: Some(armor_damages),
            injury: None,
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn injury(damages: u8, injury: Injury) -> Self {
        Self {
            damages,
            armor_damages: None,
            injury: Some(injury),
            duration_damages: None,
            knock_out: false,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn duration_damages(damages: u8, duration_damages: DurationDamages) -> Self {
        Self {
            damages,
            armor_damages: None,
            injury: None,
            duration_damages: Some(duration_damages),
            knock_out: false,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn knock_out(damages: u8) -> Self {
        Self {
            damages,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn miss_assaults(misses: TemporaryHandicap) -> Self {
        Self {
            damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: Some(misses),
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn unstoppable_assaults(count: u8) -> Self {
        Self {
            damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: None,
            unstoppable_assaults: Some(count),
            drop_weapon: false,
            weapon_damages: None,
        }
    }

    pub fn drop_weapon() -> Self {
        Self {
            damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: true,
            weapon_damages: None,
        }
    }

    pub fn damage_weapon(rupture_damages: u8) -> Self {
        Self {
            damages: 0,
            armor_damages: None,
            injury: None,
            duration_damages: None,
            knock_out: true,
            assault_misses: None,
            unstoppable_assaults: None,
            drop_weapon: false,
            weapon_damages: Some(rupture_damages),
        }
    }

    fn apply(&self, victim: &mut dyn Assailant) {
        victim.take_damage(self.damages);
        if let Some(armor_damages) = &self.armor_damages {
            armor_damages.apply(victim);
        }
        if let Some(_) = &self.injury {
            println!("WARN injuries are not applied");
        }
        if self.knock_out {
            victim.knock_out();
        }
        if let Some(misses) = &self.assault_misses {
            victim.assault_misses_mut().replace(TemporaryHandicap::new(misses.count()));
        }
        if let Some(count) = &self.unstoppable_assaults {
            victim.parry_misses_mut().replace(TemporaryHandicap::new(*count));
        }
        if self.drop_weapon {
            victim.weapon_mut().take();
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
}

impl AssaultConsequences {
    pub fn new(for_assailant: IndividualConsequences, for_victim: IndividualConsequences) -> Self {
        Self {
            for_assailant,
            for_victim,
        }
    }

    pub fn apply(&self, assailant: &mut dyn Assailant, victim: &mut dyn Assailant) {
        self.for_assailant.apply(assailant);
        self.for_victim.apply(victim);
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    }
}
