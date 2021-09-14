//! A calculator used to calculate
//! calorie content
//! ```
//! use rustybeer::calculators::calorie_counter::calculate_total_calories;
//! use rustybeer::conversions::RelativeDensity;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(
//!     178.8962039966718,
//!     calculate_total_calories(
//!         &RelativeDensity::from_specific_gravity(1.054),
//!         &RelativeDensity::from_specific_gravity(1.010)
//!     )
//! );
//!
//! ```

use crate::conversions::RelativeDensity;

/// returns calories for a 12 oz. serving size
pub fn calculate_total_calories(og: &RelativeDensity, fg: &RelativeDensity) -> f64 {
    calculate_alcohol_calories(og, fg) + calculate_carbs_calories(og, fg)
}

pub fn calculate_alcohol_calories(ord: &RelativeDensity, frd: &RelativeDensity) -> f64 {
    let og = ord.as_specific_gravity();
    let fg = frd.as_specific_gravity();
    1881.22 * fg * (og - fg) / (1.775 - og)
}

pub fn calculate_carbs_calories(ord: &RelativeDensity, frd: &RelativeDensity) -> f64 {
    let og = ord.as_specific_gravity();
    let fg = frd.as_specific_gravity();
    3550.0 * fg * ((0.1808 * og) + (0.8192 * fg) - 1.0004)
}
