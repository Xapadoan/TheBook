use shared::equipment::rupture::{Rupture, RUPTURE_MAX};
use shared::warrior::body::injury::Injury;

pub fn show_rupture(equipment: &dyn Rupture, rupture_damages: u8) -> String {
    if let Some(rup) = equipment.rupture() {
        if rupture_damages >= RUPTURE_MAX || rup + rupture_damages >= RUPTURE_MAX {
            String::from("breaks in pieces")
        } else {
            String::from("is damaged")
        }
    } else {
        String::from("cannot be damaged")
    }
}

pub fn show_lose_eye(possible_injury: &Option<Injury>) -> String {
    match possible_injury {
        Some(_) => String::from(""),
        None => String::from("Luckily, he already lost it before"),
    }
}

pub fn show_lose_finger(possible_injury: &Option<Injury>) -> String {
    match possible_injury {
        Some(_) => String::from(""),
        None => String::from("Luckily, he already lost the finger involved before"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRupture {
        rup: Option<u8>
    }
    impl Rupture for TestRupture {
        fn rupture(&self) -> &Option<u8> {
            &self.rup
        }
        fn set_rupture(&mut self, rup: Option<u8>) {
            self.rup = rup
        }
    }

    #[test]
    fn show_broken_when_rupture_equals_max() {
        let item = TestRupture { rup: Some(RUPTURE_MAX - 1) };
        let result = show_rupture(&item, 1);
        assert_eq!(result, "breaks in pieces");
    }
}
