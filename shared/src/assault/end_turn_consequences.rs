use serde::{Deserialize, Serialize};

use crate::assault::assailant::Assailant;
use crate::assault::duration_damages::TakeDurationDamages;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTurnConsequences {
    duration_damages: u8,
}

// client only
impl EndTurnConsequences {
    pub fn duration_damages(&self) -> u8 {
        self.duration_damages
    }

    pub fn apply(&self, victim: &mut dyn Assailant) {
        victim.take_damage(self.duration_damages);
    }
}

// server only
pub trait EndTurnConsequencesBuilder:
    TakeDurationDamages
{
    fn end_turn(&mut self) -> EndTurnConsequences {
        let mut duration_damages = 0;
        if let Some(damages) = self.take_duration_damages() {
            duration_damages = damages;
        }
        EndTurnConsequences {
            duration_damages,
        }
    }
}
