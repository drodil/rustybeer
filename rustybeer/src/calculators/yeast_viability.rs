//! A calculator used to estimate
//! yeast viability

pub fn calculate_yv(days: f32) -> f32 {
    let n_of_days = if days > 0.0 { days } else { 0.0 };
    97.0 * ((2.72_f32).powf(-0.008 * n_of_days))
}

pub fn calculate_cc(cc: f32, days: f32) -> f32 {
    cc * (calculate_yv(days) / 100.0)
}
