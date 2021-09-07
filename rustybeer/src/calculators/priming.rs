//! A module to determine how much,
//! and which, sugars to add at bottling
//! time for brewed beer
//! # Example
//! ```
//! use rustybeer::calculators::priming::{calculate_sugars, Sugar};
//!
//! // Arguments are in Farenheit, liters, and c02 volumes
//! let sugars = calculate_sugars(77., 5., 2.);
//!
//! println!("You can add: ");
//!
//! for sugar in sugars.iter() {
//!     println!("{}g of {}", sugar.ratio, sugar.name);
//! }
//! ```

/// A calculator to determine how much
/// priming sugar should be added at
/// bottling time for brewed beer,
/// based off a given temperature
/// Calculates the residual amount of c02
/// present in the beer due to fermentation.
/// The temperature should be given in fahrenheit
pub fn calculate_co2(fahrenheit: f64) -> f64 {
    3.0378 - 0.050062 * fahrenheit + 0.00026555 * fahrenheit.powf(2.0)
}

/// Calculates the amount of each sugar that
/// should be added at bottling time, based
/// off a given temperature (In farenheit),
/// amount of beer (In liters), and the
/// volume of c02.
/// The returned Vec of [Sugars](struct.Sugar.html)
/// can be printed or indexed as desired
pub fn calculate_sugars(fahrenheit: f64, amount: f64, co2_volumes: f64) -> Vec<Sugar> {
    let mut sugars = vec![
        Sugar::new(String::from("Table Sugar (sucrose)"), 1.0),
        Sugar::new(String::from("Corn Sugar (dextrose)"), 1.0 / 0.91),
        Sugar::new(String::from("DME - All Varieties"), 1.0 / 0.68),
        Sugar::new(String::from("DME - Laaglander"), 1.0 / 0.5),
        Sugar::new(String::from("Turbinado"), 1.0),
        Sugar::new(String::from("Demarara"), 1.0),
        Sugar::new(String::from("Corn Syrup"), 1.0 / 0.69),
        Sugar::new(String::from("Brown Sugar"), 1.0 / 0.89),
        Sugar::new(String::from("Molasses"), 1.0 / 0.71),
        Sugar::new(String::from("Maple Syrup"), 1.0 / 0.77),
        Sugar::new(String::from("Sorghum Syrup"), 1.0 / 0.69),
        Sugar::new(String::from("Honey"), 1.0 / 0.74),
        Sugar::new(String::from("Belgian Candy Syrup"), 1.0 / 0.63),
        Sugar::new(String::from("Belgian Candy Sugar"), 1.0 / 0.75),
        Sugar::new(String::from("Invert Sugar Syrup"), 1.0 / 0.91),
        Sugar::new(String::from("Black Treacle"), 1.0 / 0.87),
        Sugar::new(String::from("Rice Solids"), 1.0 / 0.79),
    ];

    let beer_co2 = calculate_co2(fahrenheit);
    let sucrose = ((co2_volumes * 2.0) - (beer_co2 * 2.0)) * 2.0 * amount;

    for sugar in sugars.iter_mut() {
        sugar.ratio *= sucrose
    }

    sugars
}

/// A sugar added at bottling time for
/// brewed beer, with a name and ratio
/// of beer to be used, in grams
#[derive(Debug, PartialEq)]
pub struct Sugar {
    pub name: String,
    pub ratio: f64,
}

impl Sugar {
    pub fn new(name: String, ratio: f64) -> Self {
        Self { name, ratio }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::assert_approx;

    #[test]
    fn priming() {
        assert_approx!(2.3466, calculate_co2(15.));
        assert_approx!(2.4556, calculate_co2(12.45));

        let stream = calculate_sugars(77., 5., 2.);

        let expected = vec![
            Sugar::new(String::from("Table Sugar (sucrose)"), 24.850561000000013),
            Sugar::new(String::from("Corn Sugar (dextrose)"), 27.308308791208802),
            Sugar::new(String::from("DME - All Varieties"), 36.54494264705884),
            Sugar::new(String::from("DME - Laaglander"), 49.701122000000026),
            Sugar::new(String::from("Turbinado"), 24.850561000000013),
            Sugar::new(String::from("Demarara"), 24.850561000000013),
            Sugar::new(String::from("Corn Syrup"), 36.015305797101476),
            Sugar::new(String::from("Brown Sugar"), 27.92197865168541),
            Sugar::new(String::from("Molasses"), 35.00079014084509),
            Sugar::new(String::from("Maple Syrup"), 32.27345584415586),
            Sugar::new(String::from("Sorghum Syrup"), 36.015305797101476),
            Sugar::new(String::from("Honey"), 33.5818391891892),
            Sugar::new(String::from("Belgian Candy Syrup"), 39.44533492063494),
            Sugar::new(String::from("Belgian Candy Sugar"), 33.13408133333335),
            Sugar::new(String::from("Invert Sugar Syrup"), 27.308308791208802),
            Sugar::new(String::from("Black Treacle"), 28.563863218390818),
            Sugar::new(String::from("Rice Solids"), 31.45640632911394),
        ];

        assert_eq!(expected, stream);
    }
}
