pub mod assault {
    pub mod assailant;
    pub mod assault_summary;
    pub mod assault_consequence;
    pub mod not_possible;
    pub mod parry_not_possible;
    pub mod parry_success;
    pub mod attack_missed;
    pub mod attack_attempt;
    pub mod attack_success;
    pub mod attack_clumsiness;
    pub mod critical_hit {
        mod critical_hit;
        mod resolve_critical_hit;
        mod deal_critical_hit;

        pub use critical_hit::CriticalHit;
        pub use resolve_critical_hit::ResolveCriticalHit;
        pub use resolve_critical_hit::ResolveCriticalHitSelf;
        pub use deal_critical_hit::DealCriticalHit;
    }
    pub mod common_traits {
        mod gouge_random_eye;
        mod miss_assaults;
        mod drop_weapon;
        mod break_weapon;
        mod damages;

        pub use miss_assaults::ResolveMissAssaults;
        pub use gouge_random_eye::ResolveGougeRandomEye;
        pub use drop_weapon::ResolveDropWeapon;
        pub use break_weapon::ResolveBreakWeapon;
        pub use damages::*;
    }
    pub mod clumsiness;
    pub mod parry_attempt;
    pub mod parry_clumsiness;
    pub mod critical_parry {
        mod critical_parry;
        mod resolve_critical_parry;
        mod deal_critical_parry;

        pub use critical_parry::CriticalParry;
        pub use resolve_critical_parry::ResolveCriticalParry;
        pub use deal_critical_parry::DealCriticalParry;
    }
    pub mod duration_damages;
    pub mod end_turn_consequences;
}

pub mod warrior {
    pub mod body {
        pub mod injury;
        pub mod body_part;
        mod body_injuries;
        mod main;

        pub use main::*;
    }
    mod main;
    mod names;
    pub use main::*;
}

pub mod equipment {
    pub mod rupture;
    pub mod protection;
    pub mod weapon;
}

pub mod random;
pub mod unique_entity;
pub mod name;
pub mod health;
pub mod stats;

pub mod tournament {
    pub mod contestant;
}

pub mod temporary_handicap {
    mod main;

    pub use main::*;
}

pub mod dice;
pub mod knock_out;

pub mod replay {
    pub mod turn_summary;
}
