use shared::assault::critical_hit::{CriticalHit, ResolveCriticalHit};
use shared::warrior::body::injury::Injuries;
use shared::warrior::body::HasBody;

use crate::common::TestAssailant;

#[test]
fn break_hand_on_intact_body() {
    let critical_hit = CriticalHit::BrokenHand;
    let victim = TestAssailant::new();
    assert!(victim.body().injuries().is_empty());

    let consequences = victim.resolve_critical_hit(0, &critical_hit);
    assert!(consequences.injury().is_some(), "No injury")
}
