use shared::stats::{Stat, StatModifier};
use shared::warrior::body::body_part::{BodyPartKind, BodySide, OptionalMutableBodyPart};
use shared::warrior::body::injury::Injury;
use shared::warrior::body::Body;

#[test]
fn foot_severed_stats_modifier() {
    let mut body = Body::new();
    body.body_part_mut(&BodyPartKind::Foot(BodySide::Left)).take();
    let injury = Injury::FootSevered(BodySide::Left);

    let attack = Stat::Attack(10);
    let parry = Stat::Parry(10);

    assert_eq!(
        injury.modify_stat(attack.clone()).value(),
        body.modify_stat(attack.clone()).value(),
    );
    assert_eq!(
        injury.modify_stat(parry.clone()).value(),
        body.modify_stat(parry.clone()).value(),
    );
}