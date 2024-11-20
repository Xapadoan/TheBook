use serde::{Deserialize, Serialize};
use shared::{equipment::protection::{OptionalMutableProtection, Protection}, warrior::body::body_part::{BodyPart, BodyPartKind, BodySide, FingerName}};
use sqlx::prelude::FromRow;

use crate::repository::sql_repository::protections::ProtectionModel;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BodyPartKindColumnData(pub BodyPartKind);
impl ToString for BodyPartKindColumnData {
    fn to_string(&self) -> String {
        match &self.0 {
            BodyPartKind::Arm(side) => side.as_str().to_string() + " Arm",
            BodyPartKind::Eye(side) => side.as_str().to_string() + " Eye",
            BodyPartKind::Finger(side, name) => side.as_str().to_string() + " " + name.as_str(),
            BodyPartKind::Foot(side) => side.as_str().to_string() + " Foot",
            BodyPartKind::Genitals => "Genitals".to_string(),
            BodyPartKind::Hand(side) => side.as_str().to_string() + " Hand",
            BodyPartKind::Head => "Head".to_string(),
            BodyPartKind::Knee(side) => side.as_str().to_string() + " Knee",
            BodyPartKind::Leg(side) => side.as_str().to_string() + " Leg",
            BodyPartKind::Torso => "Torso".to_string(),
        }
    }
}
impl From<String> for BodyPartKindColumnData {
    fn from(value: String) -> Self {
        if value == "Left Arm" {
            BodyPartKindColumnData(BodyPartKind::Arm(BodySide::Left))
        } else if value == "Right Arm" {
            BodyPartKindColumnData(BodyPartKind::Arm(BodySide::Right))
        } else if value == "Left Eye" {
            BodyPartKindColumnData(BodyPartKind::Eye(BodySide::Left))
        } else if value == "Right Eye" {
            BodyPartKindColumnData(BodyPartKind::Eye(BodySide::Right))
        } else if value == "Left Middle Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Left, FingerName::MiddleFinger))
        } else if value == "Right Middle Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Right, FingerName::MiddleFinger))
        } else if value == "Left Pinky Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Left, FingerName::PinkyFinger))
        } else if value == "Right Pinky Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Right, FingerName::PinkyFinger))
        } else if value == "Left Pointer Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Left, FingerName::PointerFinger))
        } else if value == "Right Pointer Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Right, FingerName::PointerFinger))
        } else if value == "Left Ring Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Left, FingerName::RingFinger))
        } else if value == "Right Ring Finger" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Right, FingerName::RingFinger))
        } else if value == "Left Thumb" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Left, FingerName::Thumb))
        } else if value == "Right Thumb" {
            BodyPartKindColumnData(BodyPartKind::Finger(BodySide::Right, FingerName::Thumb))
        } else if value == "Left Foot" {
            BodyPartKindColumnData(BodyPartKind::Foot(BodySide::Left))
        } else if value == "Right Foot" {
            BodyPartKindColumnData(BodyPartKind::Foot(BodySide::Right))
        } else if value == "Genitals" {
            BodyPartKindColumnData(BodyPartKind::Genitals)
        } else if value == "Left Hand" {
            BodyPartKindColumnData(BodyPartKind::Hand(BodySide::Left))
        } else if value == "Right Hand" {
            BodyPartKindColumnData(BodyPartKind::Hand(BodySide::Right))
        } else if value == "Head" {
            BodyPartKindColumnData(BodyPartKind::Head)
        } else if value == "Left Knee" {
            BodyPartKindColumnData(BodyPartKind::Knee(BodySide::Left))
        } else if value == "Right Knee" {
            BodyPartKindColumnData(BodyPartKind::Knee(BodySide::Right))
        } else if value == "Left Leg" {
            BodyPartKindColumnData(BodyPartKind::Leg(BodySide::Left))
        } else if value == "Right Leg" {
            BodyPartKindColumnData(BodyPartKind::Leg(BodySide::Right))
        } else if value == "Torso" {
            BodyPartKindColumnData(BodyPartKind::Torso)
        } else {
            panic!("Failed to create WeaponKindColumnData from String: {value}")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BodyPartModel {
    pub uuid: String,
    pub warrior_uuid: String,
    pub kind: BodyPartKindColumnData,
    pub is_broken: i8,
    pub protection_uuid: Option<String>,
}

pub struct BodyPartFinalModel {
    pub body_part_model: BodyPartModel,
    pub protection_model: Option<ProtectionModel>,
}
impl BodyPartFinalModel {
    pub fn new(
        body_part_model: BodyPartModel,
        protection_model: Option<ProtectionModel>,
    ) -> Self {
        Self {
            body_part_model,
            protection_model,
        }
    }
}
impl TryFrom<BodyPartFinalModel> for BodyPart {
    type Error = uuid::Error;

    fn try_from(value: BodyPartFinalModel) -> Result<Self, Self::Error> {
        let mut part = Self::new(value.body_part_model.kind.0);
        part.set_is_broken(value.body_part_model.is_broken == 1);
        if let Some(protection_model) = value.protection_model {
            let protection = Protection::try_from(protection_model)?;
            part.protection_mut().replace(protection);
        }

        Ok(part)
    }
}
