use regex::Regex;
use std::num::ParseFloatError;

use measurements::{Mass, Temperature, Volume};

/// Used to build new measurements::Temperature structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct TemperatureBuilder;

impl TemperatureBuilder {
    /// Creates measurements::Temperature from string
    ///
    /// Tries to figure out the temperature unit from the string. If the string value is plain
    /// number, it will be considered as Celsius. Also empty strings are considered as
    /// zero Celsius in Temperature.
    pub fn from_str(val: &str) -> Result<measurements::Temperature, ParseFloatError> {
        if val.is_empty() {
            return Ok(Temperature::from_celsius(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Z]{1})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_uppercase().as_str() {
                    "F" => Temperature::from_fahrenheit(float_val.parse::<f64>()?),
                    "C" => Temperature::from_celsius(float_val.parse::<f64>()?),
                    "K" => Temperature::from_kelvin(float_val.parse::<f64>()?),
                    "R" => Temperature::from_rankine(float_val.parse::<f64>()?),
                    _ => Temperature::from_celsius(val.parse::<f64>()?),
                },
            );
        }

        Ok(Temperature::from_celsius(val.parse::<f64>()?))
    }
}

/// Used to build new measurements::Volume structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct VolumeBuilder;

impl VolumeBuilder {
    /// Creates measurements::Volume from string
    ///
    /// Tries to figure out the volume unit from the string. If the string value is plain
    /// number, it will be considered as litres. Also empty strings are considered as
    /// zero litres in Volume.
    pub fn from_str(val: &str) -> Result<measurements::Volume, ParseFloatError> {
        if val.is_empty() {
            return Ok(Volume::from_litres(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Z]{1,3}[0-9]{0,1})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_lowercase().as_str() {
                    "cm3" => Volume::from_cubic_centimeters(float_val.parse::<f64>()?),
                    "ft3" => Volume::from_cubic_feet(float_val.parse::<f64>()?),
                    "yd3" => Volume::from_cubic_yards(float_val.parse::<f64>()?),
                    "in3" => Volume::from_cubic_inches(float_val.parse::<f64>()?),
                    "gal" => Volume::from_gallons(float_val.parse::<f64>()?),
                    "cup" => Volume::from_cups(float_val.parse::<f64>()?),
                    "tsp" => Volume::from_teaspoons(float_val.parse::<f64>()?),
                    "ml" => Volume::from_milliliters(float_val.parse::<f64>()?),
                    "m3" => Volume::from_cubic_meters(float_val.parse::<f64>()?),
                    "μl" => Volume::from_drops(float_val.parse::<f64>()?),
                    "dr" => Volume::from_drams(float_val.parse::<f64>()?),
                    "l" => Volume::from_litres(float_val.parse::<f64>()?),
                    "p" => Volume::from_pints(float_val.parse::<f64>()?),
                    "ʒ" => Volume::from_pints(float_val.parse::<f64>()?),
                    _ => Volume::from_litres(val.parse::<f64>()?),
                },
            );
        }

        Ok(Volume::from_litres(val.parse::<f64>()?))
    }
}

/// Used to build new measurements::Mass structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct MassBuilder;

impl MassBuilder {
    /// Creates measurements::Mass from string
    ///
    /// Tries to figure out the mass unit from the string. If the string value is plain
    /// number, it will be considered as grams. Also empty strings are considered as
    /// zero grams in Mass.
    pub fn from_str(val: &str) -> Result<measurements::Mass, ParseFloatError> {
        if val.is_empty() {
            return Ok(Mass::from_grams(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Zμ]{1,3})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(match caps.get(2).unwrap().as_str() {
                "ug" | "μg" => Mass::from_micrograms(float_val.parse::<f64>()?),
                "mg" => Mass::from_milligrams(float_val.parse::<f64>()?),
                "ct" => Mass::from_carats(float_val.parse::<f64>()?),
                "g" => Mass::from_grams(float_val.parse::<f64>()?),
                "kg" => Mass::from_kilograms(float_val.parse::<f64>()?),
                "T" => Mass::from_metric_tons(float_val.parse::<f64>()?),
                "gr" => Mass::from_grains(float_val.parse::<f64>()?),
                "dwt" => Mass::from_pennyweights(float_val.parse::<f64>()?),
                "oz" => Mass::from_ounces(float_val.parse::<f64>()?),
                "st" => Mass::from_stones(float_val.parse::<f64>()?),
                "lbs" => Mass::from_pounds(float_val.parse::<f64>()?),
                _ => Mass::from_grams(float_val.parse::<f64>()?),
            });
        }

        Ok(Mass::from_grams(val.parse::<f64>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{MassBuilder, TemperatureBuilder, VolumeBuilder};
    use average::assert_almost_eq;
    const DELTA: f64 = 1e-5;

    #[test]
    fn fahrenheit_from_string() {
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123F")
                .unwrap()
                .as_fahrenheit(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 F")
                .unwrap()
                .as_fahrenheit(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 f")
                .unwrap()
                .as_fahrenheit(),
            DELTA
        );
    }

    #[test]
    fn celsius_from_string() {
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123C").unwrap().as_celsius(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 C").unwrap().as_celsius(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 c").unwrap().as_celsius(),
            DELTA
        );
    }

    #[test]
    fn kelvin_from_string() {
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123K").unwrap().as_kelvin(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 K").unwrap().as_kelvin(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 k").unwrap().as_kelvin(),
            DELTA
        );
    }

    #[test]
    fn rankine_from_string() {
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123R").unwrap().as_rankine(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 R").unwrap().as_rankine(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123 r").unwrap().as_rankine(),
            DELTA
        );
    }

    #[test]
    fn default_from_string() {
        assert_almost_eq!(
            123.0,
            TemperatureBuilder::from_str("123").unwrap().as_celsius(),
            DELTA
        );

        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123").unwrap().as_litres(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123").unwrap().as_grams(),
            DELTA
        );
    }

    #[test]
    fn zero_from_string() {
        assert_almost_eq!(
            0.,
            TemperatureBuilder::from_str("").unwrap().as_celsius(),
            DELTA
        );
        assert_almost_eq!(0., VolumeBuilder::from_str("").unwrap().as_litres(), DELTA);
        assert_almost_eq!(0., MassBuilder::from_str("").unwrap().as_grams(), DELTA);
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123cm3")
                .unwrap()
                .as_cubic_centimeters(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 cm3")
                .unwrap()
                .as_cubic_centimeters(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 CM3")
                .unwrap()
                .as_cubic_centimeters(),
            DELTA
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123ml").unwrap().as_milliliters(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 ml").unwrap().as_milliliters(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 ml").unwrap().as_milliliters(),
            DELTA
        );
    }

    #[test]
    fn pints_from_string() {
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123p").unwrap().as_pints(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 p").unwrap().as_pints(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            VolumeBuilder::from_str("123 P").unwrap().as_pints(),
            DELTA
        );
    }

    #[test]
    fn micrograms_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123ug").unwrap().as_micrograms(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 ug").unwrap().as_micrograms(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123μg").unwrap().as_micrograms(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 μg").unwrap().as_micrograms(),
            DELTA
        );
    }

    #[test]
    fn milligrams_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123mg").unwrap().as_milligrams(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 mg").unwrap().as_milligrams(),
            DELTA
        );
    }

    #[test]
    fn carats_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123ct").unwrap().as_carats(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 ct").unwrap().as_carats(),
            DELTA
        );
    }

    #[test]
    fn grams_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123g").unwrap().as_grams(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 g").unwrap().as_grams(),
            DELTA
        );
    }

    #[test]
    fn kilograms_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123kg").unwrap().as_kilograms(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 kg").unwrap().as_kilograms(),
            DELTA
        );
    }

    #[test]
    fn tonnes_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123T").unwrap().as_tonnes(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 T").unwrap().as_tonnes(),
            DELTA
        );
    }

    #[test]
    fn grains_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123gr").unwrap().as_grains(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 gr").unwrap().as_grains(),
            DELTA
        );
    }

    #[test]
    fn pennyweights_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123dwt").unwrap().as_pennyweights(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 dwt").unwrap().as_pennyweights(),
            DELTA
        );
    }

    #[test]
    fn ounces_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123oz").unwrap().as_ounces(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 oz").unwrap().as_ounces(),
            DELTA
        );
    }

    #[test]
    fn pounds_from_string() {
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123lbs").unwrap().as_pounds(),
            DELTA
        );
        assert_almost_eq!(
            123.0,
            MassBuilder::from_str("123 lbs").unwrap().as_pounds(),
            DELTA
        );
    }
}
