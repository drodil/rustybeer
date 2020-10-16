//! A calculator used to calculate
//! calorie content
//! ```
//! use rustybeer::calories::calculate_total_calories;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(186.686187744140625, calculate_total_calories(1.052, 1.040));
//! ```

// formulas is for 12 ounces
pub fn calculate_total_calories(og: f32, fg: f32) -> f32 {
    calculate_alcohol_calories(og, fg) + calculate_carbs_calories(og, fg)
}
pub fn calculate_alcohol_calories(og: f32, fg: f32) -> f32 {
    1881.22 * fg * (og - fg) / (1.775 - og)
}

pub fn calculate_carbs_calories(og: f32, fg: f32) -> f32 {
    3550.0 * fg * ((0.1808 * og) + (0.8192 * fg) - 1.0004)
}

// tc: total calories, cv: custom volume (ml)
// converting cal per 12 oz to ml then multiplying by cv
pub fn convert_to_serving_size(tc: f32, cv: f32) -> f32 {
    (tc / 354.8823) * cv
}