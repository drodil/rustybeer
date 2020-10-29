//! A calculator used to calculate final gravity
//! ```
//! use rustybeer::calculators::fg::calculate_fg;
//!
//! // Takes the arguments original gravity and yeast attenuation
//! assert_eq!(1.0125, calculate_fg(1.050, 75));
//! ```

pub fn calculate_fg(og: f32, att: u8) -> f32 {
    og - (att as f32 / 100.0) * (og - 1.0)
}
