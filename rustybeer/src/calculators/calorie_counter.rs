//! A calculator used to calculate
//! calorie content
//! ```
//! use rustybeer::calculators::calorie_counter::calculate_total_calories;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(178.89642, calculate_total_calories(1.054, 1.010));
//! ```

/// returns calories for a 12 oz. serving size
pub fn calculate_total_calories(og: f32, fg: f32) -> f32 {
    calculate_alcohol_calories(og, fg) + calculate_carbs_calories(og, fg)
}

pub fn calculate_alcohol_calories(og: f32, fg: f32) -> f32 {
    1881.22 * fg * (og - fg) / (1.775 - og)
}

pub fn calculate_carbs_calories(og: f32, fg: f32) -> f32 {
    3550.0 * fg * ((0.1808 * og) + (0.8192 * fg) - 1.0004)
}
