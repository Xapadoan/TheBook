use super::body_part::{BodyPart, BodyPartKind, BodySide, OptionalBodyPart, OptionalMutableBodyPart, ALL_FINGERS};
use super::injury::{Injuries, Injury};
use super::Body;

fn both_legs_severed(body: &dyn OptionalBodyPart) -> bool {
    body.body_part(&BodyPartKind::Leg(BodySide::Right)).is_none() &&
    body.body_part(&BodyPartKind::Leg(BodySide::Left)).is_none()
}
    
fn one_leg_severed(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    if both_legs_severed(body) {
        return false;
    }
    body.body_part(&BodyPartKind::Leg(side.clone())).is_none()
}

fn both_legs_broken(body: &dyn OptionalBodyPart) -> bool {
    let left_is_broken = body.body_part(&BodyPartKind::Leg(BodySide::Left))
        .as_ref()
        .is_some_and(|part| part.is_broken());
    let right_is_broken = body.body_part(&BodyPartKind::Leg(BodySide::Right))
        .as_ref()
        .is_some_and(|part| part.is_broken());
    right_is_broken && left_is_broken
}

fn one_leg_broken(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    if both_legs_broken(body) {
        return false;
    }
    body.body_part(&BodyPartKind::Leg(side.clone()))
        .as_ref()
        .is_some_and(|part| part.is_broken())
}

fn knee_dislocated(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    body.body_part(&BodyPartKind::Knee(side.clone()))
        .as_ref()
        .is_some_and(|part| part.is_broken())
}

fn foot_severed(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    let leg_not_severed = body.body_part(&BodyPartKind::Leg(side.clone())).is_some();
    let foot_is_severed = body.body_part(&BodyPartKind::Foot(side.clone())).is_none();
    leg_not_severed && foot_is_severed
}

fn foot_smashed(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    body.body_part(&BodyPartKind::Foot(side.clone()))
        .as_ref()
        .is_some_and(|part| part.is_broken())
}

fn both_eyes_gouged(body: &dyn OptionalBodyPart) -> bool {
    body.body_part(&BodyPartKind::Eye(BodySide::Left)).is_none() &&
    body.body_part(&BodyPartKind::Eye(BodySide::Right)).is_none()
}

fn one_eye_gouged(body: &dyn OptionalBodyPart, side: &BodySide) -> bool {
    if both_eyes_gouged(body) {
        return false;
    }
    body.body_part(&BodyPartKind::Eye(side.clone())).is_none()
}

impl Injuries for Body {
    fn injuries(&self) -> Vec<Injury> {
        let mut injuries = vec![];
        if both_legs_severed(self) {
            injuries.push(Injury::BothLegsSevered);
        }
        if one_leg_severed(self, &BodySide::Left) {
            injuries.push(Injury::OneLegSevered(BodySide::Left));
        }
        if one_leg_severed(self, &BodySide::Right) {
            injuries.push(Injury::OneLegSevered(BodySide::Right));
        }
        if both_legs_broken(self) {
            injuries.push(Injury::BothLegsBroken);
        }
        if one_leg_broken(self, &BodySide::Left) {
            injuries.push(Injury::OneLegBroken(BodySide::Left));
        }
        if one_leg_broken(self, &BodySide::Right) {
            injuries.push(Injury::OneLegBroken(BodySide::Right));
        }
        if knee_dislocated(self, &BodySide::Left) {
            injuries.push(Injury::KneeDislocated(BodySide::Left));
        }
        if knee_dislocated(self, &BodySide::Right) {
            injuries.push(Injury::KneeDislocated(BodySide::Right));
        }
        if foot_severed(self, &BodySide::Left) {
            injuries.push(Injury::FootSevered(BodySide::Left));
        }
        if foot_severed(self, &BodySide::Right) {
            injuries.push(Injury::FootSevered(BodySide::Right));
        }
        if foot_smashed(self, &BodySide::Left) {
            injuries.push(Injury::FootSmashed(BodySide::Left));
        }
        if foot_smashed(self, &BodySide::Right) {
            injuries.push(Injury::FootSmashed(BodySide::Right));
        }
        if both_eyes_gouged(self) {
            injuries.push(Injury::BothEyesGouged);
        }
        if one_eye_gouged(self, &BodySide::Left){
            injuries.push(Injury::OneEyeGouged(BodySide::Left))
        }
        if one_eye_gouged(self, &BodySide::Right){
            injuries.push(Injury::OneEyeGouged(BodySide::Right))
        }
        if self.body_part(&BodyPartKind::Genitals).as_ref().is_some_and(|part| part.is_broken()) {
            injuries.push(Injury::GenitalsCrushed)
        }
        if let Some(arm) = self.body_part(&BodyPartKind::Arm(BodySide::Left)) {
            if arm.is_broken() {
                injuries.push(Injury::LeftArmBroken);
            }
            if let Some(hand) = self.body_part(&BodyPartKind::Hand(BodySide::Left)) {
                if hand.is_broken() && !arm.is_broken() {
                    injuries.push(Injury::LeftHandBroken)
                }
                for finger_name in ALL_FINGERS {
                    if let None = self.body_part(&BodyPartKind::Finger(BodySide::Left, finger_name.clone())) {
                        injuries.push(Injury::FingerSevered(BodySide::Left, finger_name.clone()))
                    }
                }
            } else {
                injuries.push(Injury::LeftHandSevered)
            }
        } else {
            injuries.push(Injury::LeftArmSevered)
        }
        if let Some(arm) = self.body_part(&BodyPartKind::Arm(BodySide::Right)) {
            if arm.is_broken() {
                injuries.push(Injury::RightArmBroken);
            }
            if let Some(hand) = self.body_part(&BodyPartKind::Hand(BodySide::Right)) {
                if hand.is_broken() && !arm.is_broken() {
                    injuries.push(Injury::RightHandBroken)
                }
                for finger_name in ALL_FINGERS {
                    if let None = self.body_part(&BodyPartKind::Finger(BodySide::Right, finger_name.clone())) {
                        injuries.push(Injury::FingerSevered(BodySide::Right, finger_name.clone()))
                    }
                }
            } else {
                injuries.push(Injury::RightHandSevered)
            }
        } else {
            injuries.push(Injury::RightArmSevered)
        }
        injuries
    }

    fn add_injury(&mut self, injury: Injury) -> Vec<BodyPart> {
        let mut severed_parts = vec![];
        match injury {
            Injury::FingerSevered(side, finger) => self.remove_part(
                &BodyPartKind::Finger(side.clone(), finger.clone()),
                &mut severed_parts,
            ),
            Injury::FootSevered(side) => self.remove_part(
                &BodyPartKind::Foot(side.clone()),
                &mut severed_parts,
            ),
            Injury::FootSmashed(side) => self.break_part(&BodyPartKind::Foot(side.clone())),
            Injury::GenitalsCrushed => self.break_part(&BodyPartKind::Genitals),
            Injury::KneeDislocated(side) => self.break_part(&BodyPartKind::Knee(side.clone())),
            Injury::LeftArmBroken => self.break_part(&BodyPartKind::Arm(BodySide::Left)),
            Injury::LeftArmSevered => self.remove_part(
                &BodyPartKind::Arm(BodySide::Left),
                &mut severed_parts,
            ),
            Injury::LeftElbowDislocated => { eprintln!("[WARN] {:?} have no corresponding BodyPartKind", self) },
            Injury::LeftHandBroken => self.break_part(&BodyPartKind::Hand(BodySide::Left)),
            Injury::LeftHandSevered => self.remove_part(
                &BodyPartKind::Hand(BodySide::Left),
                &mut severed_parts,
            ),
            Injury::LeftShoulderDislocated => { eprintln!("[WARN] {:?} have no corresponding BodyPartKind", self)},
            Injury::OneEyeGouged(side) => self.remove_part(
                &BodyPartKind::Eye(side.clone()),
                &mut severed_parts,
            ),
            Injury::OneLegBroken(side) => self.break_part(&BodyPartKind::Leg(side.clone())),
            Injury::OneLegSevered(side) => self.remove_part(
                &BodyPartKind::Leg(side.clone()),
                &mut severed_parts,
            ),
            Injury::RightArmBroken => self.break_part(&BodyPartKind::Arm(BodySide::Right)),
            Injury::RightArmSevered => self.remove_part(
                &BodyPartKind::Arm(BodySide::Right),
                &mut severed_parts,
            ),
            Injury::RightElbowDislocated => { eprintln!("[WARN] {:?} have no corresponding BodyPartKind", self) },
            Injury::RightHandBroken => self.break_part(&BodyPartKind::Hand(BodySide::Right)),
            Injury::RightHandSevered => self.remove_part(
                &BodyPartKind::Hand(BodySide::Right),
                &mut severed_parts,
            ),
            Injury::RightShoulderDislocated => { eprintln!("[WARN] {:?} have no corresponding BodyPartKind", self)},
            _ => { eprintln!("[WARN] {:?} should not be passed to add_injury", self) },
        }
        severed_parts
    }
}

#[cfg(test)]
mod tests {
    use crate::warrior::body::body_part::OptionalMutableBodyPart;

    use super::*;

    #[test]
    fn injuries_both_legs_severed() {
        let mut body = Body::new();
        let left = BodyPartKind::Leg(BodySide::Left);
        let right = BodyPartKind::Leg(BodySide::Right);

        body.body_part_mut(&left).take();
        body.body_part_mut(&right).take();
        assert!(body.body_part(&left).is_none());
        assert!(body.body_part(&right).is_none());

        assert!(body.injuries().contains(&Injury::BothLegsSevered));
    }

    #[test]
    fn injuries_both_legs_broken() {
        let mut body = Body::new();
        let left = BodyPartKind::Leg(BodySide::Left);
        let right = BodyPartKind::Leg(BodySide::Right);

        body.body_part_mut(&left).as_mut().unwrap().set_is_broken(true);
        body.body_part_mut(&right).as_mut().unwrap().set_is_broken(true);

        assert!(body.body_part(&left).as_ref().unwrap().is_broken());
        assert!(body.body_part(&right).as_ref().unwrap().is_broken());

        assert!(body.injuries().contains(&Injury::BothLegsBroken));
    }

    #[test]
    fn one_foot_severed() {
        let mut body = Body::new();
        body.body_part_mut(&BodyPartKind::Foot(BodySide::Left)).take();
        assert!(body.body_part(&BodyPartKind::Foot(BodySide::Left)).is_none());

        let injuries = body.injuries();

        assert_eq!(injuries.len(), 1);
        assert!(injuries.contains(&Injury::FootSevered(BodySide::Left)))
    }
}