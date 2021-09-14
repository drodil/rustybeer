//! A calculator used to calculate final gravity
//! ```
//! use rustybeer::conversions::RelativeDensity;
//! use rustybeer::calculators::fg::calculate_fg;
//!
//! // Takes the arguments original gravity and yeast attenuation
//! assert_eq!(RelativeDensity::from_specific_gravity(1.0125), calculate_fg(&RelativeDensity::from_specific_gravity(1.050), 75));
//! ```

use crate::conversions::RelativeDensity;

pub fn calculate_fg(ord: &RelativeDensity, att: u8) -> RelativeDensity {
    let og = ord.as_specific_gravity();
    RelativeDensity::from_specific_gravity(og - (att as f64 / 100.0) * (og - 1.0))
}
