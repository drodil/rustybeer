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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;

    #[test]
    fn alcohol_volume_weight() {
        assert_approx!(4., calculate_abv_abw(5.));
        assert_approx!(0., calculate_abv_abw(0.));
        assert_approx!(400., calculate_abv_abw(500.));

        assert_approx!(3.945, calculate_abv_abw_density(5., 1.));
        assert_approx!(5., calculate_abv_abw_density(5., 0.789));
        assert_approx!(0.789, calculate_abv_abw_density(5., 5.));
        assert_approx!(100., calculate_abv_abw_density(5., 0.03945));

        assert_approx!(6.25, calculate_abw_abv(5.));
        assert_approx!(0., calculate_abw_abv(0.));
        assert_approx!(625., calculate_abw_abv(500.));

        assert_approx!(6.3371, calculate_abw_abv_density(5., 1.));
        assert_approx!(5., calculate_abw_abv_density(5., 0.789));
        assert_approx!(31.6857, calculate_abw_abv_density(5., 5.));
        assert_approx!(0., calculate_abw_abv_density(5., 0.));

        assert_approx!(50., calculate_alc_vol(1000., 5.));
        assert_approx!(28.4, calculate_alc_vol(568., 5.));
        assert_approx!(0., calculate_alc_vol(0., 5.));
        assert_approx!(0.00005, calculate_alc_vol(0.001, 5.));
        assert_approx!(5000., calculate_alc_vol(100000., 5.));

        assert_approx!(0., calculate_alc_vol(1000., 0.));
        assert_approx!(7.89, calculate_alc_vol(1000., 0.789));
        assert_approx!(1000., calculate_alc_vol(1000., 100.));

        assert_approx!(39.45, calculate_alc_weight(1000., 5.));
        assert_approx!(22.4076, calculate_alc_weight(568., 5.));
        assert_approx!(0., calculate_alc_weight(0., 5.));
        assert_approx!(0.00003945, calculate_alc_weight(0.001, 5.));
        assert_approx!(3945., calculate_alc_weight(100000., 5.));

        assert_approx!(0., calculate_alc_weight(1000., 0.));
        assert_approx!(6.22521, calculate_alc_weight(1000., 0.789));
        assert_approx!(789., calculate_alc_weight(1000., 100.));
    }
}
