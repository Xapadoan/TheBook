use crate::fight_mechanics::{ApplyDamageModifier, TakeDamage};
use crate::modifiers::Modifier;

use super::body_parts::BodyPartKind;

pub trait Destroyable {
    fn is_destroyed(&self) -> bool;
    fn destroy(&mut self);
}

#[derive(Debug)]
pub enum ProtectionKind {
    Gambeson,
    Jacket,
    Plastron,
    Greave,
    Armlet,
    ChainMail,
    Helm,
}

#[derive(Debug)]
pub struct Protection {
    kind: ProtectionKind,
    dmg_modifier: Modifier,
    rupture: Option<u8>,
    is_destroyed: bool,
}

impl Protection {
    pub fn new(kind: ProtectionKind) -> Self {
        match kind {
            ProtectionKind::Armlet => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                is_destroyed: false,
            },
            ProtectionKind::ChainMail => Self {
                kind: ProtectionKind::ChainMail,
                dmg_modifier: Modifier::new(-4),
                rupture: Some(3),
                is_destroyed: false,
            },
            ProtectionKind::Gambeson => Self {
                kind: ProtectionKind::Gambeson,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(4),
                is_destroyed: false,
            },
            ProtectionKind::Greave => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-1),
                rupture: Some(5),
                is_destroyed: false,
            },
            ProtectionKind::Helm => Self {
                kind: ProtectionKind::Helm,
                dmg_modifier: Modifier::new(0),
                rupture: Some(5),
                is_destroyed: false,
            },
            ProtectionKind::Jacket => Self {
                kind: ProtectionKind::Jacket,
                dmg_modifier: Modifier::new(-2),
                rupture: Some(5),
                is_destroyed: false,
            },
            ProtectionKind::Plastron => Self {
                kind: ProtectionKind::Armlet,
                dmg_modifier: Modifier::new(-3),
                rupture: Some(4),
                is_destroyed: false,
            },
        }
    }

    pub fn can_be_equipped_on(&self, body_part: BodyPartKind) -> bool {
        match self.kind {
            ProtectionKind::Armlet => {
                match body_part {
                    BodyPartKind::Arm(_) => true,
                    _ => false,
                }
            },
            ProtectionKind::ChainMail => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Gambeson => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Greave => {
                match body_part {
                    BodyPartKind::Leg(_) => true,
                    _ => false,
                }
            },
            ProtectionKind::Helm => {
                match body_part {
                    BodyPartKind::Head => true,
                    _ => false,
                }
            },
            ProtectionKind::Jacket => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
            ProtectionKind::Plastron => {
                match body_part {
                    BodyPartKind::Torso => true,
                    _ => false,
                }
            },
        }
    }
}

impl Destroyable for Protection {
    fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    fn destroy(&mut self) {
        self.is_destroyed = true;
    }
}

impl TakeDamage for Protection {
    fn take_damage(&mut self, damage: u8) {
        if self.rupture.is_none() {
            return;
        }
        let mut rup = self.rupture.unwrap();
        if damage < rup {
            rup -= damage;
        } else {
            self.destroy();
            rup = 0;
        }
        self.rupture = Some(rup);
    }
}

impl ApplyDamageModifier for Protection {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        if self.is_destroyed() {
            base
        } else {
            self.dmg_modifier.apply(base)
        }
    }
}
