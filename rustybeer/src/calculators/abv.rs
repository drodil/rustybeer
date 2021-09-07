//! A calculator used to calculate
//! alcohol by volume
//! ```
//! use rustybeer::calculators::abv::calculate_abv;
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;

    #[test]
    fn abv() {
        assert_approx!(1050., calculate_abv(10., 2.));
        assert_approx!(39.5548, calculate_abv(0.3026, 0.00123));
    }
}
