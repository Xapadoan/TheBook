use crate::assault::assault_consequence::IndividualConsequences;
use crate::random::Random;
use crate::warrior::body::body_part::{BodyPartKind, BodySide, OptionalBodyPart};
use crate::warrior::body::injury::Injury;
use crate::warrior::body::HasMutableBody;

pub trait ResolveGougeRandomEye: HasMutableBody {
    fn resolve_gouge_random_eye(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        let injury = if let None = self.body().body_part(&BodyPartKind::Eye(affected_side.other())) {
            Injury::BothEyesGouged
        } else {
            Injury::OneEyeGouged(affected_side)
        };
        IndividualConsequences::injury(damages + 5, injury)
    }
}