/// A module for calculating IBU using Tinseth Formula:
///
/// IBUs = decimal alpha acid utilization * mg/l of added alpha acids
///
///
/// See:
/// https://www.realbeer.com/hops/research.html
/// http://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
/// https://straighttothepint.com/ibu-calculator/
/// https://www.brewersfriend.com/2010/02/27/hops-alpha-acid-table-2009/
///

/// Internal function to calculate Aplha Acid Utilization (Tinseth formula)
/// given Boil Time and Wort Original Gravity
/// # Arguments
///
/// * `wort_gravity`: wort Original Gravity
/// * `time_mins`: boil time (min)
///
fn _calculate_utilization(wort_gravity: f64, time_mins: u32) -> f64 {
    let bigness_factor = 1.65 * f64::powf(0.000125, wort_gravity - 1.0);
    let boil_time_factor = (1.0 - f64::exp(-0.04 * (time_mins as f64))) / 4.15;
    bigness_factor * boil_time_factor
}

/// Internal function to calculate IBU contributed by single hop addition,
///
/// # Arguments
///
/// * `weight_grams`: weight of the hop addition (gm)
/// * `alpha_acid_percentage`: AA% of the hop variety
/// * `time_mins`: boil time (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: the wort original gravity
///
fn _calculate_ibu_single_hop(
    weight_grams: f64,
    alpha_acid_percentage: f64,
    time_mins: u32,
    finished_volume_liters: f64,
    gravity_boil: f64,
    utilization_multiplier: f64,
) -> f64 {
    let mg_per_liter_added_aa =
        (alpha_acid_percentage * weight_grams * 1000.0) / finished_volume_liters;
    let decimal_alpha_acid_utilization =
        _calculate_utilization(gravity_boil, time_mins) * utilization_multiplier;
    mg_per_liter_added_aa * decimal_alpha_acid_utilization
}

/// An enum of hop types
#[derive(Debug, Copy, Clone)]
pub enum HopAdditionType {
    /// Whole, default
    Whole,
    // Plugs, same utilization as whole hops
    Plug,
    /// Pellets, 10% higher utilization
    Pellet,
}

impl Default for HopAdditionType {
    fn default() -> Self {
        HopAdditionType::Whole
    }
}

/// A representation of one hop addition
///
/// Example:
/// ```
/// use rustybeer::calculators::ibu::{HopAddition, HopAdditionType};
/// // Centennial (8.5% AA) Pellets: 7g - 60 min
/// HopAddition {
///     weight_grams: 7.,
///     alpha_acid_percentage: 0.085,
///     time_mins: 60,
///     hop_type: HopAdditionType::Pellet
/// };
///```
///
#[derive(Debug, Copy, Clone)]
// TODO: YAML/JSON serialization
pub struct HopAddition {
    /// the weight of the hop addition (gm)
    pub weight_grams: f64,
    /// AA% of the hop variety
    pub alpha_acid_percentage: f64,
    /// boil time (min)
    pub time_mins: u32,
    /// type of hop added: whole or pellets. [default() = HopAdditionType::Whole]
    pub hop_type: HopAdditionType,
}

impl HopAddition {
    pub fn new(
        weight_grams: f64,
        alpha_acid_percentage: f64,
        time_mins: u32,
        hop_type: HopAdditionType,
    ) -> Self {
        Self {
            weight_grams,
            alpha_acid_percentage,
            time_mins,
            hop_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeIbuError;

/// Calculates IBU contributed by hop additions
///
/// # Arguments
///
/// * `hop_additions`: the added hops weights (g), AA%, and boil time (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: wort original gravity
///
/// # Examples
///
/// * Target Batch Size: 20 liters
/// * Original Gravity: 1.050
/// * Cascade (6.4% AA): 28g - 45 mins
///
/// ```
/// use rustybeer::calculators::ibu::HopAddition;
/// use rustybeer::calculators::ibu::calculate_ibu;
/// assert!( (18.972_316 - calculate_ibu(vec![HopAddition::new(28.0, 0.064, 45, Default::default())], 20.0, 1.050)).abs() < 0.01);
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
                h.time_mins,
                finished_volume_liters,
                gravity_boil,
                match h.hop_type {
                    HopAdditionType::Whole | HopAdditionType::Plug => 1.,
                    HopAdditionType::Pellet => 1.1,
                },
            )
        })
        .sum()
}

/// Calculates the needed amount of bittering hop to reach a target IBU for given variety alpha
/// acid percentage and boil time of the hop
///
/// # Arguments
///
/// * `hop_additions`: Optional other flavor or aroma hops additions
/// * `bittering_alpha_acid_percentage`: the alpha acid percentage of the bittering hop variety
/// * `bittering_time_mins`: Optional boil time of the bittering hop (min)
/// * `finished_volume_liters`: volume of the final wort (liters)
/// * `gravity_boil`: wort original gravity
/// * `target_ibu`: target IBU
///
/// # Examples
///
/// * Target Batch Size: 22 liters
/// * Original Gravity: 1.058
/// * Target IBU: 17
/// * Centennial (8.5% AA) hops to be added for 60min boil
/// * No other hops additions
/// ```
/// use rustybeer::calculators::ibu::calculate_bittering_weight;
/// let bittering = calculate_bittering_weight(None, 0.085, None, 22., 1.058, 17.);
/// assert!( (20.50 - bittering.unwrap()).abs() < 0.01);
/// ```
///
/// With addition of 20gm of Centennial (8.5% AA) for 60min boil,
/// can't get IBU down to just 10
///
/// ```{.should_panic}
/// use rustybeer::calculators::ibu::calculate_bittering_weight;
/// use rustybeer::calculators::ibu::HopAddition;
/// let bittering = calculate_bittering_weight(Some(vec![
///     HopAddition {
///         weight_grams: 20.,
///         alpha_acid_percentage: 0.085,
///         time_mins: 60,
///         hop_type: Default::default()}]),
///     0.085, None, 22., 1.058, 10.);
///
/// bittering.expect("Too low IBU target");
/// ```
///
pub fn calculate_bittering_weight(
    hop_additions: Option<Vec<HopAddition>>,
    bittering_alpha_acid_percentage: f64,
    bittering_time_mins: Option<u32>,
    finished_volume_liters: f64,
    gravity_boil: f64,
    target_ibu: f64,
) -> Result<f64, NegativeIbuError> {
    let bittering_ibu = match hop_additions {
        Some(h) => target_ibu - calculate_ibu(h, finished_volume_liters, gravity_boil),
        None => target_ibu,
    };

    match bittering_ibu.is_sign_positive() {
        true => {
            let bittering_time = bittering_time_mins.unwrap_or(60);
            let bittering_alpha_acid_utilization =
                _calculate_utilization(gravity_boil, bittering_time);

            let bittering_weight = (bittering_ibu * finished_volume_liters)
                / (bittering_alpha_acid_utilization * bittering_alpha_acid_percentage)
                / 1000.0;

            Ok(bittering_weight)
        }
        false => Err(NegativeIbuError),
    }
}

#[cfg(test)]
pub mod test {
    use super::{
        calculate_bittering_weight, calculate_ibu, HopAddition, HopAdditionType, NegativeIbuError,
        _calculate_ibu_single_hop, _calculate_utilization,
    };
    use crate::calculators::utilization_test_vector;
    use average::assert_almost_eq;

    #[test]
    fn utilization() {
        let test_vector: utilization_test_vector::TestVector =
            utilization_test_vector::get_vector();
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
            _calculate_ibu_single_hop(7.0, 0.085, 15, 22.0, 1.058, 1.),
            0.01
        );
    }

    #[test]
    fn multiple_hops_ibu() {
        assert_almost_eq!(
            5.76,
            calculate_ibu(
                vec![
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Whole),
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Whole)
                ],
                22.0,
                1.058
            ),
            0.01
        );
    }

    #[test]
    fn pellet_hops_ibu() {
        // 6.336 = 5.76 * 1.1
        assert_almost_eq!(
            6.336,
            calculate_ibu(
                vec![
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Pellet),
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Pellet)
                ],
                22.0,
                1.058
            ),
            0.01
        );
    }

    #[test]
    #[should_panic]
    fn negative_ibu() {
        calculate_bittering_weight(
            Some(vec![HopAddition::new(
                20.0,
                0.085,
                60,
                HopAdditionType::Whole,
            )]),
            0.085,
            None,
            22.0,
            1.058,
            10.,
        )
        .expect("too low IBU");
    }

    #[test]
    fn bitter_hops_weight() -> Result<(), NegativeIbuError> {
        assert_almost_eq!(
            13.2611,
            calculate_bittering_weight(
                Some(vec![
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Whole),
                    HopAddition::new(7.0, 0.085, 15, HopAdditionType::Plug)
                ]),
                0.085,
                Some(60),
                22.0,
                1.058,
                16.76,
            )?,
            0.001
        );
        Ok(())
    }

    #[test]
    fn zero_hops_ibu() {
        assert_almost_eq!(0., calculate_ibu(vec![], 22.0, 1.058), f64::EPSILON);
    }
}
