//! This calculates how much wort is needed to be
//! diluted or boiled down to hit a certain gravity
//! ```
//! use rustybeer::conversions::RelativeDensity;
//! use measurements::Volume;
//! use rustybeer::calculators::diluting::calculate_new_volume;
//!
//! // This displays how a target volume based off the
//! // current volume, current gravity, and target gravity
//! assert_eq!(Volume::from_liters(6.357142857142855), calculate_new_volume(&RelativeDensity::from_specific_gravity(1.089), &Volume::from_liters(50.), &RelativeDensity::from_specific_gravity(1.70)));
//! ```

use crate::conversions::RelativeDensity;
use measurements::Volume;

/// Calculates the new gravity based off a current gravity,
/// a current volume of wort, and a target volume of wort
pub fn calculate_new_gravity(
    current_gravity: &RelativeDensity,
    current_volume: &Volume,
    target_volume: &Volume,
) -> RelativeDensity {
    RelativeDensity::from_specific_gravity(
        (current_gravity.as_specific_gravity() - 1.0)
            * (current_volume.as_liters() / target_volume.as_liters())
            + 1.0,
    )
}

/// Calculates the new volume based off a current
/// gravity, a current volume, and a target gravity
pub fn calculate_new_volume(
    current_gravity: &RelativeDensity,
    current_volume: &Volume,
    target_gravity: &RelativeDensity,
) -> Volume {
    Volume::from_liters(
        current_volume.as_liters() * (current_gravity.as_specific_gravity() - 1.)
            / (target_gravity.as_specific_gravity() - 1.),
    )
}

#[cfg(test)]
pub mod tests {

    #[test]
    fn boil_off() {}

    #[test]
    fn diluting() {}
}
