pub enum RuptureTestResult {
    Fail,
    Success
}

pub trait HasRupture {
    fn is_destroyed(&self) -> bool;
    fn rupture_test(&self) -> RuptureTestResult;
    fn damage_rupture(&mut self, damage: u8);
}