use rand::Rng;

pub const RUPTURE_MAX: u8 = 6;

#[derive(Debug)]
pub enum RuptureTestResult {
    Success,
    Fail,
}

pub trait Rupture {
    fn rupture(&self) -> &Option<u8>;
    fn set_rupture(&mut self, rup: Option<u8>);
    fn rupture_test(&self) -> RuptureTestResult {
        let res = rand::thread_rng().gen_range(0..=6);
        match self.rupture() {
            Some(rupture) => if res > *rupture {
                RuptureTestResult::Success
            } else {
                RuptureTestResult::Fail
            },
            None => RuptureTestResult::Success
        }
    }
    fn damage_rupture(&mut self, damages: u8) {
        if let Some(rupture) = self.rupture() {
            if damages > RUPTURE_MAX || *rupture + damages > RUPTURE_MAX {
                self.set_rupture(Some(RUPTURE_MAX));
            } else {
                self.set_rupture(Some(*rupture + damages));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::u8;

    use super::*;

    struct TestEquipment {
        rup: Option<u8>,
    }
    impl Rupture for TestEquipment {
        fn rupture(&self) -> &Option<u8> {
            &self.rup
        }
        fn set_rupture(&mut self, rup: Option<u8>) {
            self.rup = rup;
        }
    }

    #[test]
    fn damage_rupture_does_not_overflow() {
        let mut equipment = TestEquipment { rup: Some(4) };
        equipment.damage_rupture(u8::MAX);
        assert_eq!(RUPTURE_MAX, equipment.rupture().unwrap());
    }
}

impl PartialEq for RuptureTestResult {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RuptureTestResult::Fail => match other {
                RuptureTestResult::Fail => true,
                _ => false,
            },
            RuptureTestResult::Success => match other {
                RuptureTestResult::Success => true,
                _ => false,
            },
        }
    }
}
