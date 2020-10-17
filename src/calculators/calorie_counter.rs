//! A calculator used to calculate
//! calorie content
//! ```
//! use rustybeer::calories::calculate_total_calories;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(186.686187744140625, calculate_total_calories(1.052, 1.040));
//! ```

pub fn calculate_total_calories(og: f32, fg: f32, cv: f32) -> f32 {
    calculate_alcohol_calories(og, fg, cv) + calculate_carbs_calories(og, fg, cv) 
}

pub fn calculate_alcohol_calories(og: f32, fg: f32, cv: f32) -> f32 {
    (1881.22 * fg * (og - fg) / (1.775 - og) / 354.8823) * cv
}

pub fn calculate_carbs_calories(og: f32, fg: f32, cv: f32) -> f32 {
    (3550.0 * fg * ((0.1808 * og) + (0.8192 * fg) - 1.0004) / 354.8823) * cv
}