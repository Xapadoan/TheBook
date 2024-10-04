use shared::equipment::{protection::{Protection, ProtectionKind}, rupture::{Rupture, RuptureTestResult, RUPTURE_MAX}};

pub fn unbreakable_protection(kind: ProtectionKind) -> Protection {
    let mut protection = Protection::new(kind);
    protection.set_rupture(None);
    protection
}

#[test]
fn unbreakable_protection_never_fails_rupture_test() {
    let glove = unbreakable_protection(ProtectionKind::Gloves);
    let mut i: u8 = 0;
    while i < 100 {
        assert_eq!(glove.rupture_test(), RuptureTestResult::Success);
        i += 1;
    }
}

pub fn unreliable_protection(kind: ProtectionKind) -> Protection {
    let mut protection = Protection::new(kind);
    protection.set_rupture(Some(RUPTURE_MAX));
    protection
}

#[test]
fn unreliable_protection_always_fails_rupture_test() {
    let glove = unreliable_protection(ProtectionKind::Gloves);
    let mut i: u8 = 0;
    while i < 100 {
        assert_eq!(glove.rupture_test(), RuptureTestResult::Fail);
        i += 1;
    }
}
