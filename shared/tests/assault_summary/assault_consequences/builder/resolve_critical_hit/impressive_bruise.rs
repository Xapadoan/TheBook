use shared::{assault::critical_hit::{CriticalHit, ResolveCriticalHit}, health::MutableHealth};

use crate::common::TestAssailant;

#[test]
fn impressive_bruise() {
    let critical_hit = CriticalHit::ImpressiveBruise;
    let victim = TestAssailant::new();
    assert_eq!(victim.health().current(), victim.health().max());

    let consequences = victim.resolve_critical_hit(0, &critical_hit);
    assert_ne!(consequences.raw_damages(), 0);
}