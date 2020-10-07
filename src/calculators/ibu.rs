/// IBU calculator based on https://www.realbeer.com/hops/research.html
/// Other links:
/// http://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
/// https://straighttothepint.com/ibu-calculator/
/// https://www.brewersfriend.com/2010/02/27/hops-alpha-acid-table-2009/
///

/// A representation of one hop addition
#[derive(Debug, Copy, Clone, PartialEq)]
// TODO: YAML/JSON serialization
// TODO: FromStr
// TODO: PartialEq
pub struct HopAddition {
    /// the weight of the hop addition (gm)
    pub weight_grams: f64,
    /// AA% of the hop variety
    pub alpha_acid_percentage: f64,
    /// boil time (min)
    pub time_in_mins: u32,
}

impl HopAddition {
    pub fn new(weight_grams: f64, alpha_acid_percentage: f64, time_in_mins: u32) -> Self {
        Self {
            weight_grams,
            alpha_acid_percentage,
            time_in_mins,
        }
    }
}

/// Internal function to calculate Aplha Acid Utilization (Tinseth formula)
/// given Boil Time and Wort Original Gravity
/// # Arguments
///
/// * `wort_gravity`: wort Original Gravity
/// * `time_in_mins`: boil time (min)
///
fn _calculate_utilization(wort_gravity: f64, time_in_mins: u32) -> f64 {
    let bigness_factor = 1.65 * f64::powf(0.000125, wort_gravity - 1.0);
    let boil_time_factor = (1.0 - f64::exp(-0.04 * (time_in_mins as f64))) / 4.15;
    bigness_factor * boil_time_factor
}

/// Internal function to calculate IBU contributed by single hop addition,
/// using Glenn Tinseth's formula:
/// IBUs = decimal alpha acid utilization * mg/l of added alpha acids
///
/// # Arguments
///
/// * `weight_grams`: weight of the hop addition (gm)
/// * `alpha_acid_percentage`: AA% of the hop variety
/// * `time_in_mins`: boil time (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: the wort original gravity
///
fn _calculate_ibu_single_hop(
    weight_grams: f64,
    alpha_acid_percentage: f64,
    time_in_mins: u32,
    finished_volume_liters: f64,
    gravity_boil: f64,
) -> f64 {
    let mg_per_liter_added_aa =
        (alpha_acid_percentage * weight_grams * 1000.0) / finished_volume_liters;
    let decimal_alpha_acid_utilization = _calculate_utilization(gravity_boil, time_in_mins);
    mg_per_liter_added_aa * decimal_alpha_acid_utilization
}

/// Calculates IBU contributed by hop additions (using Glenn Tinseth's formula)
///
/// # Arguments
///
/// * `hop_additions`: the added hops weights (g), AA%, and boil time (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: wort original gravity
///
/// # Examples
///
/// Target Batch Size: 20 liters
/// Original Gravity: 1.050
/// 28g Hops - 6.4% AA @ 45 mins
/// ```
/// use rustybeer::calculators::ibu::HopAddition;
/// use rustybeer::calculators::ibu::calculate_ibu;
/// assert!( (18.972_316 - calculate_ibu(vec![HopAddition::new(28.0, 0.064, 45)], 20.0, 1.050)).abs() < 0.01);
/// ```
///
pub fn calculate_ibu(
    hop_additions: Vec<HopAddition>,
    finished_volume_liters: f64,
    gravity_boil: f64,
) -> f64 {
    hop_additions
        .into_iter()
        .map(|h| {
            _calculate_ibu_single_hop(
                h.weight_grams,
                h.alpha_acid_percentage,
                h.time_in_mins,
                finished_volume_liters,
                gravity_boil,
            )
        })
        .sum()
}

/// Calculates the weight of bittering hop with given alpha acid percentage and boil time
/// to reach a target IBU
/// # Arguments
///
/// * `hop_additions`: other flavor or aroma hops additions
/// * `bittering_alpha_acid_percentage`: the alpha acid percentage of the bittering hop variety
/// * `bittering_time_in_mins`: Optional boil time of the bittering hop (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: wort original gravity
/// * `target_ibu`: target IBU
///
/// # Examples
///
/// ```
/// use rustybeer::calculators::ibu::calculate_bittering_weight_for_ibu;
/// //assert!( (20.50 - calculate_weight_for_ibu(17.0, 0.085, 60, 22.0, 1.058)).abs() < 0.01)
/// ```
///
pub fn calculate_bittering_weight_for_ibu(
    hop_additions: Vec<HopAddition>,
    bittering_alpha_acid_percentage: f64,
    bittering_time_in_mins: Option<u32>,
    finished_volume_liters: f64,
    gravity_boil: f64,
    target_ibu: f64,
) -> HopAddition {
    let bittering_ibu = match hop_additions.len() {
        0 => target_ibu,
        _ => target_ibu - calculate_ibu(hop_additions, finished_volume_liters, gravity_boil),
    };
    let bittering_time = bittering_time_in_mins.unwrap_or(60);
    let bittering_alpha_acid_utilization = _calculate_utilization(gravity_boil, bittering_time);

    let bittering_weight = (bittering_ibu * finished_volume_liters)
        / (bittering_alpha_acid_utilization * bittering_alpha_acid_percentage)
        / 1000.0;

    HopAddition::new(
        bittering_weight,
        bittering_alpha_acid_percentage,
        bittering_time,
    )
}

#[cfg(test)]
pub mod test {
    use super::{
        _calculate_ibu_single_hop, _calculate_utilization, calculate_bittering_weight_for_ibu,
        calculate_ibu, HopAddition,
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
            for (boiling_time_idx, boiling_time) in test_vector.boiling_time.iter().enumerate() {
                let ut = _calculate_utilization(*og, *boiling_time);
                assert_almost_eq!(test_vector.utilization[boiling_time_idx][og_idx], ut, 0.001);
            }
        }
    }

    #[test]
    fn single_hop_ibu() {
        assert_almost_eq!(
            2.88,
            _calculate_ibu_single_hop(7.0, 0.085, 15, 22.0, 1.058),
            0.01
        );
    }

    #[test]
    fn bitter_hops_weight() {
        assert_eq!(
            HopAddition {
                weight_grams: 13.2611_2216_6795_023,
                alpha_acid_percentage: 0.085,
                time_in_mins: 60
            },
            calculate_bittering_weight_for_ibu(
                vec![
                    HopAddition::new(7.0, 0.085, 15),
                    HopAddition::new(7.0, 0.085, 15)
                ],
                0.085,
                Some(60),
                22.0,
                1.058,
                16.76,
            )
        );
    }

    #[test]
    fn multiple_hops_ibu() {
        assert_almost_eq!(
            5.76,
            calculate_ibu(
                vec![
                    HopAddition::new(7.0, 0.085, 15),
                    HopAddition::new(7.0, 0.085, 15)
                ],
                22.0,
                1.058
            ),
            0.01
        );
    }

    #[test]
    fn zero_hops_ibu() {
        assert!((0. - calculate_ibu(vec![], 22.0, 1.058)).abs() < f64::EPSILON);
    }
}
