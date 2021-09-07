//! A calculator used to estimate
//! yeast viability

pub fn calculate_yv(days: f32) -> f32 {
    let n_of_days = if days > 0.0 { days } else { 0.0 };
    97.0 * ((2.72_f32).powf(-0.008 * n_of_days))
}

pub fn calculate_cc(cc: f32, days: f32) -> f32 {
    cc * (calculate_yv(days) / 100.0)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;
    #[test]
    fn yeast_viability() {
        assert_approx!(970.0, calculate_cc(1000.0, 0.0));
        assert_approx!(115.048_26, calculate_cc(123.45, 5.0));
        assert_approx!(0.0, calculate_cc(9001.0, 3650.0));
        assert_approx!(97.0, calculate_yv(0.0));
        assert_approx!(65.004_616, calculate_yv(50.0));
        assert_approx!(0.0, calculate_yv(3650.0));
    }
}
