use crate::warrior::body::body_part::MayTargetBodyPart;
use crate::warrior::body::HasBody;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::body::injury::MayBeInjured;
use crate::equipment::{MayHaveTestedRupture, RuptureTestResult};
use crate::warrior::Name;
use super::attack::critical_hit::{CriticalHitResult, CriticalHitKind};
use super::parry::critical_parry::CriticalParryResult;

pub trait ShowAction {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name,
        V: Name + HasBody;
}

impl ShowAction for CriticalHitResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name,
        V: Name + HasBody,
    {
        match self.kind() {
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                println!(
                    "{} hits {} heavily, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::BrokenArm | CriticalHitKind::BrokenHand | CriticalHitKind::BrokenLeg => {
                println!("{} broke {}'s {}", assailant.name(), victim.name(), self.target_body_part().unwrap());
            }
            CriticalHitKind::CrushedGenitals => {
                let genitals = self.target_body_part().unwrap();
                if victim.body().body_part(genitals).is_severed() {
                    println!(
                        "{} smashed the air when {}'s {} should have been",
                        assailant.name(),
                        victim.name(),
                        genitals,
                    )
                } else {
                    println!("{} crushed {}'s genitals", assailant.name(), victim.name());
                }
            }
            CriticalHitKind::DeepIncision => {
                println!("{} cut {} deeply", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredGenitals => {
                println!("{} severed {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitKind::GougedEye => {
                let eye = victim.body().body_part(self.target_body_part().unwrap());
                if eye.is_severed() {
                    println!("{}'s {} is already gouged", victim.name(), eye.kind());
                } else {
                    println!("{} gouged {}'s eye", assailant.name(), victim.name());
                }
            }
            CriticalHitKind::HeartInjury => {
                println!("{} pierced {}'s heart", assailant.name(), victim.name());
            }
            CriticalHitKind::ImpressiveBruise => {
                println!("{} bruised {} heavily", assailant.name(), victim.name());
            }
            CriticalHitKind::ImpressiveBruiseAndLimbDislocation => {
                println!(
                    "{} bruised {} heavily, dislocating a limb",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::ImpressiveWoundAndArmorDamage => {
                println!(
                    "{} wounded {} deeply, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::KneeDislocation => {
                println!("{} dislocated {}'s knee", assailant.name(), victim.name());
            }
            CriticalHitKind::KnockedOut => {
                println!("{} knocked {} out", assailant.name(), victim.name());
            }
            CriticalHitKind::OpenSkullFacture => {
                println!("{} opened {}'s skull wide", assailant.name(), victim.name());
            }
            CriticalHitKind::PartOfTheArmorIsDestroyed => {
                match self.rupture_test_result() {
                    Some(result) => match result {
                        RuptureTestResult::Fail => println!(
                            "{} destroyed {}'s {}",
                            assailant.name(),
                            victim.name(),
                            self.display_protection_or_limb(victim),
                        ),
                        RuptureTestResult::Success => println!(
                            "{} hits {} savagely, damaging his {}",
                            assailant.name(),
                            victim.name(),
                            self.display_protection_or_limb(victim),
                        )
                    },
                    None => println!(
                        "{} hits {}'s unprotected {}.",
                        assailant.name(),
                        victim.name(),
                        self.target_body_part().unwrap(),
                    )
                }
            }
            CriticalHitKind::PreciseHitAndArmorDamage => {
                println!(
                    "{} hit {} precisely, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::ReallyDeepIncision => {
                println!("{} cut {} really deep", assailant.name(), victim.name());
            }
            CriticalHitKind::RibFacture => {
                println!("{} fractured {}'s rib", assailant.name(), victim.name());
            }
            CriticalHitKind::SeriousHeadInjury => {
                println!("{} cut through {}'s head", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredArm | CriticalHitKind::SeveredFoot | CriticalHitKind::SeveredHand | CriticalHitKind::SeveredLeg => {
                let body_part = victim.body().body_part(self.target_body_part().unwrap());
                if body_part.is_severed() {
                    println!("{} slashed right where {}'s {} should be", assailant.name(), victim.name(), body_part.kind());
                } else {
                    println!("{} severed {}'s {}", assailant.name(), victim.name(), body_part.kind());
                }
            }
            CriticalHitKind::SmashedFoot => {
                let body_part = victim.body().body_part(self.target_body_part().unwrap());
                if body_part.is_severed() {
                    println!(
                        "{} smashed the ground where {}'s {} should have been",
                        assailant.name(),
                        victim.name(),
                        body_part.kind(),
                    )
                } else {
                    println!("{} smashed {}'s {}", assailant.name(), victim.name(), body_part.kind());
                }
            }
            CriticalHitKind::VitalOrganCrushed => {
                println!(
                    "{} crushed one of {}'s vital organs",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::VitalOrganDamage => {
                println!(
                    "{} damaged one of {}'s vital organs",
                    assailant.name(),
                    victim.name()
                );
            }
        }
    }
}

impl ShowAction for CriticalParryResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name,
        V: HasBody + Name
    {
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if assailant.weapon().is_none() {
                    println!("{} has no weapon", assailant.name());
                } else {
                    match rupture_test_result {
                        RuptureTestResult::Fail => println!("{} broke {}'s weapon", victim.name(), assailant.name()),
                        RuptureTestResult::Success => println!("{} damaged {}'s weapon", victim.name(), assailant.name()),
                    }
                }
            },
            CriticalParryResult::AssailantCriticalHit => println!("{} finds a great counter", victim.name()),
            CriticalParryResult::AssailantSelfCriticalHit => println!("{}'s weapon is repelled against him", assailant.name()),
            CriticalParryResult::AssailantDropsWeapon => {
                if assailant.weapon().is_some() {
                    println!("{} dropped his weapon", assailant.name())
                } else {
                    println!("{} has no weapon", assailant.name())
                }
            },
            CriticalParryResult::AssailantFalls => println!("{} falls to the ground", assailant.name()),
            CriticalParryResult::AssailantHit => println!("{} counters {}'s attack", victim.name(), assailant.name()),
            CriticalParryResult::AssailantRepelled => println!("{} repelled {}", victim.name(), assailant.name()),
            CriticalParryResult::AssailantTrips => println!("{} trips", assailant.name()),
            CriticalParryResult::RegularParry => println!("{} parried successfully", victim.name())
        }
    }
}
