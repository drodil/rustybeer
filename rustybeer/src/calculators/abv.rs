//! A calculator used to calculate
//! alcohol by volume
//! ```
//! use rustybeer::abv::calculate_abv;
//!
//! // Takes the arguments original gravity
//! // and final gravity
//! assert_eq!(1050., calculate_abv(10., 2.));
//! ```

pub fn calculate_abv(og: f32, fg: f32) -> f32 {
    (og - fg) * 131.25
}

pub fn calculate_fg(og: f32, abv: f32) -> f32 {
    og - (abv / 131.25)
}
