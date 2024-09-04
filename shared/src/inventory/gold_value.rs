use crate::{
    equipment::{
        rupture::{Rupture, RUPTURE_MAX}, weapon::Weapon
    },
    stats::StatModifier,
};

use super::Item;

pub trait GoldValue {
    fn gold_value(&self) -> u32;
}

impl GoldValue for Weapon {
    fn gold_value(&self) -> u32 {
        let rupture_value = equipment_rupture_value(self.rupture());
        let dmg = if self.is_two_handed() { 5 } else { 3 };
        let mut dmg_value = 30;
        let mut i = self.additional_damages();
        while i > dmg {
            dmg_value *= 2;
            i -= 1;
        }

        let mut value = if self.is_two_handed() { 100 } else { 50 };
        value += rupture_value;
        value += dmg_value;
        value = modify_value(self, value);

        value
    }
}

impl GoldValue for Item {
    fn gold_value(&self) -> u32 {
        match self {
            Self::Protection(_) => panic!("Not yet implemented"),
            Self::Weapon(weapon) => weapon.gold_value(),
        }
    }
}

fn equipment_rupture_value(rupture: &Option<u8>) -> u32 {
    match rupture {
        None => 500,
        Some(rup) => {
            let mut i = *rup;
            let mut value = 30;
            while i < RUPTURE_MAX {
                value *= 2;
                i += 1;
            }
            value
        }
    }
}

fn stat_modifier_value(mut modifier: i8, threshold: i8, mut low_value: i32, mut hight_value: i32) -> i32 {
    if modifier < threshold {
        while modifier < threshold {
            low_value *= 2;
            modifier += 1;
        }
        low_value
    } else if modifier > threshold {
        while modifier > threshold {
            hight_value *= 2;
            modifier -= 1;
        }
        hight_value
    } else {
        0
    }
}

trait StatsValueThresholds {
    fn base_value_thresholds(&self) -> (i8, i8);
}

impl StatsValueThresholds for Weapon {
    fn base_value_thresholds(&self) -> (i8, i8) {
        if self.is_two_handed() {
            (-2, -3)
        } else {
            (0, 0)
        }
    }
}

fn modify_value<T: StatModifier + StatsValueThresholds>(item: &T, base: u32) -> u32 {
    let mut new_value = base;
    let (at, pr) = item.base_value_thresholds();
    let at_value = stat_modifier_value(
        item.attack_mod(),
        at,
        -10,
        40,
    );
    let pr_value = stat_modifier_value(
        item.parry_mod(),
        pr,
        -10,
        30,
    );
    new_value = match new_value.checked_add_signed(at_value) {
        Some(v) => v,
        None => if at_value < 0 { 0 } else { u32::MAX },
    };
    new_value = match new_value.checked_add_signed(pr_value) {
        Some(v) => v,
        None => if pr_value < 0 { 0 } else { u32::MAX },
    };

    new_value
}
