//! A calculator used to estimate
//! yeast viability

pub fn calculate_yv(days: f32) -> f32 {
    let n_of_days = if days > 0.0 { days } else { 0.0 };
    let res = 97.0 - (0.7 * n_of_days);
    if res > 0.0 {
        res
    } else {
        0.0
    }
}

pub fn calculate_cc(cc: f32, days: f32) -> f32 {
    cc * (calculate_yv(days) / 100.0)
}
