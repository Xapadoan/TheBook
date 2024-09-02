use serde::{Deserialize, Serialize};

use crate::{assault::common_traits::ReduceDamages, name::Name};

use super::rupture::Rupture;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Protection {
    amount: u8,
    rupture: Option<u8>,
    name: String,
}

pub trait OptionalMutableProtection {
    fn protection(&self) -> &Option<Protection>;
    fn protection_mut(&mut self) -> &mut Option<Protection>;
}

impl ReduceDamages for Protection {
    fn reduce_damages(&self, damages: u8) -> u8 {
        if damages > self.amount {
            damages - self.amount
        } else {
            0
        }
    }
}

impl Rupture for Protection {
    fn rupture(&self) -> &Option<u8> {
        &self.rupture
    }

    fn set_rupture(&mut self, rup: Option<u8>) {
        self.rupture = rup;
    }
}

impl Name for Protection {
    fn name(&self) -> &str {
        &self.name
    }
}
