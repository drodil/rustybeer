//! A module to determine how much,
//! and which, sugars to add at bottling
//! time for brewed beer
//! # Example
//! ```
//! use rustybeer::calculators::priming::{calculate_sugars, Sugar};
//! use rustybeer::measurements::{Temperature, Volume, Mass};
//!
//! let sugars = calculate_sugars(
//!     &Temperature::from_fahrenheit(77.),
//!     &Volume::from_liters(5.),
//!     2.
//! );
//!
//! println!("You can add: ");
//!
//! for sugar in sugars.iter() {
//!     println!("{}g of {}", sugar.ratio.as_grams(), sugar.name);
//! }
//! ```

use measurements::{Mass, Temperature, Volume};

/// A calculator to determine how much
/// priming sugar should be added at
/// bottling time for brewed beer,
/// based off a given temperature
/// Calculates the residual amount of c02
/// present in the beer due to fermentation.
pub fn calculate_co2(temp: &Temperature) -> f64 {
    3.0378 - 0.050062 * temp.as_fahrenheit() + 0.00026555 * temp.as_fahrenheit().powf(2.0)
}

/// Calculates the amount of each sugar that
/// should be added at bottling time, based
/// off a given temperature,
/// amount of beer, and the
/// volume of c02.
/// The returned Vec of [Sugars](struct.Sugar.html)
/// can be printed or indexed as desired
pub fn calculate_sugars(temp: &Temperature, amount: &Volume, co2_volumes: f64) -> Vec<Sugar> {
    let mut sugars = vec![
        Sugar::new(String::from("Table Sugar (sucrose)"), Mass::from_grams(1.0)),
        Sugar::new(
            String::from("Corn Sugar (dextrose)"),
            Mass::from_grams(1.0 / 0.91),
        ),
        Sugar::new(
            String::from("DME - All Varieties"),
            Mass::from_grams(1.0 / 0.68),
        ),
        Sugar::new(
            String::from("DME - Laaglander"),
            Mass::from_grams(1.0 / 0.5),
        ),
        Sugar::new(String::from("Turbinado"), Mass::from_grams(1.0)),
        Sugar::new(String::from("Demarara"), Mass::from_grams(1.0)),
        Sugar::new(String::from("Corn Syrup"), Mass::from_grams(1.0 / 0.69)),
        Sugar::new(String::from("Brown Sugar"), Mass::from_grams(1.0 / 0.89)),
        Sugar::new(String::from("Molasses"), Mass::from_grams(1.0 / 0.71)),
        Sugar::new(String::from("Maple Syrup"), Mass::from_grams(1.0 / 0.77)),
        Sugar::new(String::from("Sorghum Syrup"), Mass::from_grams(1.0 / 0.69)),
        Sugar::new(String::from("Honey"), Mass::from_grams(1.0 / 0.74)),
        Sugar::new(
            String::from("Belgian Candy Syrup"),
            Mass::from_grams(1.0 / 0.63),
        ),
        Sugar::new(
            String::from("Belgian Candy Sugar"),
            Mass::from_grams(1.0 / 0.75),
        ),
        Sugar::new(
            String::from("Invert Sugar Syrup"),
            Mass::from_grams(1.0 / 0.91),
        ),
        Sugar::new(String::from("Black Treacle"), Mass::from_grams(1.0 / 0.87)),
        Sugar::new(String::from("Rice Solids"), Mass::from_grams(1.0 / 0.79)),
    ];

    let beer_co2 = calculate_co2(temp);
    let sucrose = ((co2_volumes * 2.0) - (beer_co2 * 2.0)) * 2.0 * amount.as_liters();

    for sugar in sugars.iter_mut() {
        sugar.ratio = Mass::from_grams(sugar.ratio.as_grams() * sucrose);
    }

    sugars
}

/// A sugar added at bottling time for
/// brewed beer, with a name and ratio
/// of beer to be used
#[derive(Debug, PartialEq)]
pub struct Sugar {
    pub name: String,
    pub ratio: Mass,
}

impl Sugar {
    pub fn new(name: String, ratio: Mass) -> Self {
        Self { name, ratio }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;
    use measurements::{Mass, Temperature, Volume};

    #[test]
    fn priming() {
        assert_approx!(2.3466, calculate_co2(&Temperature::from_fahrenheit(15.)));
        assert_approx!(2.4556, calculate_co2(&Temperature::from_fahrenheit(12.45)));

        let stream = calculate_sugars(
            &Temperature::from_fahrenheit(77.),
            &Volume::from_liters(5.),
            2.,
        );

        let expected = vec![
            Sugar::new(
                String::from("Table Sugar (sucrose)"),
                Mass::from_grams(24.850561000000013),
            ),
            Sugar::new(
                String::from("Corn Sugar (dextrose)"),
                Mass::from_grams(27.308308791208802),
            ),
            Sugar::new(
                String::from("DME - All Varieties"),
                Mass::from_grams(36.54494264705884),
            ),
            Sugar::new(
                String::from("DME - Laaglander"),
                Mass::from_grams(49.701122000000026),
            ),
            Sugar::new(
                String::from("Turbinado"),
                Mass::from_grams(24.850561000000013),
            ),
            Sugar::new(
                String::from("Demarara"),
                Mass::from_grams(24.850561000000013),
            ),
            Sugar::new(
                String::from("Corn Syrup"),
                Mass::from_grams(36.015305797101476),
            ),
            Sugar::new(
                String::from("Brown Sugar"),
                Mass::from_grams(27.92197865168541),
            ),
            Sugar::new(
                String::from("Molasses"),
                Mass::from_grams(35.00079014084509),
            ),
            Sugar::new(
                String::from("Maple Syrup"),
                Mass::from_grams(32.27345584415586),
            ),
            Sugar::new(
                String::from("Sorghum Syrup"),
                Mass::from_grams(36.015305797101476),
            ),
            Sugar::new(String::from("Honey"), Mass::from_grams(33.5818391891892)),
            Sugar::new(
                String::from("Belgian Candy Syrup"),
                Mass::from_grams(39.44533492063494),
            ),
            Sugar::new(
                String::from("Belgian Candy Sugar"),
                Mass::from_grams(33.13408133333335),
            ),
            Sugar::new(
                String::from("Invert Sugar Syrup"),
                Mass::from_grams(27.308308791208802),
            ),
            Sugar::new(
                String::from("Black Treacle"),
                Mass::from_grams(28.563863218390818),
            ),
            Sugar::new(
                String::from("Rice Solids"),
                Mass::from_grams(31.45640632911394),
            ),
        ];

        assert_eq!(expected, stream);
    }
}
