//! A calculator used to calculate
//! alcohol by volume
//! ```
//! use rustybeer::conversions::RelativeDensity;
//! use rustybeer::calculators::abv::calculate_abv;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(
//!     4.987500000000004,
//!     calculate_abv(
//!         &RelativeDensity::from_specific_gravity(1.050),
//!         &RelativeDensity::from_specific_gravity(1.012)
//!     )
//! );
//! ```

use crate::conversions::RelativeDensity;

pub fn calculate_abv(og: &RelativeDensity, fg: &RelativeDensity) -> f64 {
    (og.as_specific_gravity() - fg.as_specific_gravity()) * 131.25
}

pub fn calculate_fg(og: &RelativeDensity, abv: f64) -> f64 {
    og.as_specific_gravity() - (abv / 131.25)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;

    #[test]
    fn abv() {
        assert_approx!(
            4.9875,
            calculate_abv(
                &RelativeDensity::from_specific_gravity(1.050),
                &RelativeDensity::from_specific_gravity(1.012)
            )
        );
    }

    #[test]
    fn fg() {
        assert_approx!(
            1.2504761904761905,
            calculate_fg(&RelativeDensity::from_specific_gravity(1.30), 6.5)
        );
    }
}
