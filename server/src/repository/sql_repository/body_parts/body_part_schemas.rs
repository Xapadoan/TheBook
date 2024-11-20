use shared::warrior::body::body_part::BodyPartKind;
use uuid::Uuid;

use super::BodyPartKindColumnData;

pub struct CreateBodyPartSchema {
    pub uuid: Uuid,
    pub warrior_uuid: Uuid,
    pub kind: BodyPartKindColumnData,
}
impl CreateBodyPartSchema {
    pub fn new(kind: &BodyPartKind, warrior_uuid: &Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            warrior_uuid: warrior_uuid.clone(),
            kind: BodyPartKindColumnData(kind.clone()),
        }
    }
}

pub struct UpdateBodyPartSchema {
    pub is_broken: i8,
    pub protection_uuid: Option<String>,
}
