//! This calculates how much wort is needed to be
//! diluted or boiled down to hit a certain gravity
//! ```
//! use rustybeer::calculators::diluting::calculate_new_volume;
//!
//! // This displays how a target volume based off the
//! // current volume, current gravity, and target gravity
//! assert_eq!(17.28, calculate_new_volume(3.16, 50., 7.25));
//! ```

/// Calculates the new gravity based off a current gravity,
/// a current volume of wort, and a target volume of wort
pub fn calculate_new_gravity(current_gravity: f32, current_volume: f32, target_volume: f32) -> f32 {
    (current_gravity - 1.0) * (current_volume / target_volume) + 1.0
}

/// Calculates the new volume based off a current
/// gravity, a current volume, and a target gravity
pub fn calculate_new_volume(current_gravity: f32, current_volume: f32, target_gravity: f32) -> f32 {
    current_volume * (current_gravity - 1.) / (target_gravity - 1.)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;

    #[test]
    fn boil_off() {
        assert_approx!(2., calculate_new_volume(2., 2., 2.));
        assert_approx!(15., calculate_new_volume(7., 5., 3.));
        assert_approx!(11., calculate_new_gravity(7., 5., 3.));
        assert_approx!(69.5714, calculate_new_gravity(4., 3.2, 0.14));
    }

    #[test]
    fn diluting() {
        assert_approx!(14.1625, calculate_new_gravity(9.1, 5.2, 3.2));
        assert_approx!(4.5305, calculate_new_gravity(9.1, 3.16, 7.25));
        assert_approx!(2.0, calculate_new_volume(2., 2., 2.));
        assert_approx!(15., calculate_new_volume(7., 5., 3.));
    }
}
