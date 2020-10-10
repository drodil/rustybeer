//! This calculates how much wort is needed to be
//! diluted or boiled down to hit a certain gravity
//! ```
//! use rustybeer::diluting::calculate_new_volume;
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
