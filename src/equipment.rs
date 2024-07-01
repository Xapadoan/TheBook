#[derive(Debug)]
pub enum RuptureTestResult {
    Fail,
    Success
}

pub trait HasRupture {
    fn is_destroyed(&self) -> bool;
    fn rupture_test(&self) -> RuptureTestResult;
    fn damage_rupture(&mut self, damage: u8);
}

pub trait MayHaveTestedRupture {
    fn rupture_test_result(&self) -> Option<&RuptureTestResult>;
}

pub trait MayHaveRuptureDamage {
    fn rupture_damage(&self) -> Option<u8>;
}

pub const RUPTURE_MAX: u8 = 5;
