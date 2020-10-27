//! A calculator used to calculate
//! alcohol by weight from alcohol by volume and vice versa
//! ```
//! use rustybeer::calculators::alcohol_volume_weight::calculate_abv_abw;
//!
//! // Takes the argument percent (ABV)
//! assert_eq!(4., calculate_abv_abw(5.));
//! ```

static ETHANOL_DENSITY: f64 = 0.789;

pub fn calculate_abv_abw(percent: f64) -> f64 {
    percent * 0.8
}

pub fn calculate_abv_abw_density(percent: f64, total_density: f64) -> f64 {
    percent * (ETHANOL_DENSITY / total_density)
}

pub fn calculate_abw_abv(percent: f64) -> f64 {
    percent * 1.25
}

pub fn calculate_abw_abv_density(percent: f64, total_density: f64) -> f64 {
    percent * (total_density / ETHANOL_DENSITY)
}

pub fn calculate_alc_vol(total_volume: f64, abv: f64) -> f64 {
    (abv / 100.0) * total_volume
}

pub fn calculate_alc_weight(total_volume: f64, abv: f64) -> f64 {
    ((abv / 100.0) * total_volume) * ETHANOL_DENSITY
}
