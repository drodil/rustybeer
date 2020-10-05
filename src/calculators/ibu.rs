/// IBU calculator based on https://www.realbeer.com/hops/research.html
/// Other links:
/// http://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
/// https://straighttothepint.com/ibu-calculator/
/// https://www.brewersfriend.com/2010/02/27/hops-alpha-acid-table-2009/
///

/// Calculates Aplha Acid Utilization (Tinseth formula) vs Boil Time and Wort Original Gravity
/// # Arguments
///
/// * `wort_gravity`: Wort Original Gravity
/// * `time_in_mins`: Boil time in minutes
///
/// # Examples
///
/// ```
/// use rustybeer::calculators::ibu::calculate_utilization;
/// assert!( (0.231 - calculate_utilization(1.05, 60)).abs() < 0.001)
/// ```
///
pub fn calculate_utilization(wort_gravity: f64, time_in_mins: u32) -> f64 {
    let bigness_factor = 1.65 * f64::powf(0.000125, wort_gravity - 1.0);
    let boil_time_factor = (1.0 - f64::exp(-0.04 * (time_in_mins as f64))) / 4.15;
    bigness_factor * boil_time_factor
}

/// Calculates IBU contributed by single hop addition using Glenn Tinseth's formula:
/// IBUs = decimal alpha acid utilization * mg/l of added alpha acids
///
/// # Arguments
///
/// * `weight_grams`: the weight of the hop addition (in grams)
/// * `alpha_acid_percentage`: the alpha acid percentage of the hop variety
/// * `time_in_mins`: Boil time in minutes
/// * `finished_volume_liters`: the volume of the final wort in liters
/// * `gravity_boil`: the wort original gravity
///
/// # Examples
///
/// Target Batch Size: 20 liters
/// Original Gravity: 1.050
/// 28g Hops - 6.4% AA @ 45 mins
/// ```
/// use rustybeer::calculators::ibu::calculate_ibu_single_hop;
/// assert!( (18.97 - calculate_ibu_single_hop(28.0, 0.064, 45, 20.0, 1.050)).abs() < 0.01);
/// ```
///
pub fn calculate_ibu_single_hop(weight_grams: f64, alpha_acid_percentage: f64, time_in_mins: u32, finished_volume_liters: f64, gravity_boil: f64) -> f64 {
    let mg_per_liter_added_aa = (alpha_acid_percentage * weight_grams * 1000.0) / finished_volume_liters;
    let decimal_alpha_acid_utilization = calculate_utilization(gravity_boil, time_in_mins);
    mg_per_liter_added_aa * decimal_alpha_acid_utilization
}


/// Calculates the Weight in grams of bitter hops for target IBU contribution
/// # Arguments
///
/// * `ibu`: the target IBU
/// * `alpha_acid_percentage`: the alpha acid percentage of the hop variety
/// * `time_in_mins`: Boil time in minutes
/// * `finished_volume_liters`: the volume of the final wort in liters
/// * `gravity_boil`: the wort original gravity
///
/// # Examples
///
/// ```
/// use rustybeer::calculators::ibu::calculate_weight_for_ibu;
/// assert!( (20.50 - calculate_weight_for_ibu(17.0, 0.085, 60, 22.0, 1.058)).abs() < 0.01)
/// ```
///
pub fn calculate_weight_for_ibu(ibu: f64, alpha_acid_percentage: f64, time_in_mins: u32, finished_volume_liters: f64, gravity_boil: f64) -> f64 {
    let decimal_alpha_acid_utilization = calculate_utilization(gravity_boil, time_in_mins);
    (ibu * finished_volume_liters) / (decimal_alpha_acid_utilization * alpha_acid_percentage) / 1000.0
}



#[cfg(test)]
pub mod test {
    use super::{
        calculate_ibu_single_hop,
        calculate_weight_for_ibu,
        calculate_utilization
    };
    use crate::calculators::utilization_test_vector;
    use average::assert_almost_eq;

    pub struct TestVector {
        pub og: Vec<f64>,
        pub boiling_time: Vec<u32>,
        pub utilization: Vec<Vec<f64>>,
    }

    #[test]
    fn utilization() {
        let test_vector: TestVector = utilization_test_vector::get_vector();
        for (og_idx, og) in test_vector.og.iter().enumerate() {
            for (boiling_time_idx, boiling_time) in  test_vector.boiling_time.iter().enumerate() {
                let ut = calculate_utilization(*og, *boiling_time);
                assert_almost_eq!( test_vector.utilization[boiling_time_idx][og_idx], ut, 0.001);
            }
        }
    }

    #[test]
    fn single_hop_ibu() {
        assert_almost_eq!( 2.88, calculate_ibu_single_hop(7.0, 0.085, 15, 22.0, 1.058), 0.01);
    }

    #[test]
    fn bitter_hops_weight() {
        assert_almost_eq!( 26.73, calculate_weight_for_ibu(11.0, 0.085, 15, 22.0, 1.058), 0.01);
    }

}