use shared::equipment::rupture::{Rupture, RUPTURE_MAX};
use shared::warrior::body::injury::Injury;

pub fn show_rupture(equipment: &dyn Rupture, rupture_damages: u8) -> String {
    if let Some(rup) = equipment.rupture() {
        if rupture_damages > RUPTURE_MAX || rup + rupture_damages > RUPTURE_MAX {
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
