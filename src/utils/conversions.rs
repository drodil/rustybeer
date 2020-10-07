use regex::Regex;
use std::num::ParseFloatError;

use measurements::{Temperature, Volume};

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

#[cfg(test)]
mod tests {
    use super::TemperatureBuilder;
    use super::VolumeBuilder;
    const DELTA: f64 = 1e-5;

    fn abs(x: f64) -> f64 {
        if x > 0.0 {
            x
        } else {
            -x
        }
    }

    fn assert_almost_equal(a: f64, b: f64) {
        if !((abs(a - b) / a) < DELTA) {
            panic!("assertion failed: {:?} != {:?}", a, b);
        }
    }

    #[test]
    fn fahrenheit_from_string() {
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123F")
                .unwrap()
                .as_fahrenheit(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 F")
                .unwrap()
                .as_fahrenheit(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 f")
                .unwrap()
                .as_fahrenheit(),
        );
    }

    #[test]
    fn celsius_from_string() {
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123C").unwrap().as_celsius(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 C").unwrap().as_celsius(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 c").unwrap().as_celsius(),
        );
    }

    #[test]
    fn kelvin_from_string() {
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123K").unwrap().as_kelvin(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 K").unwrap().as_kelvin(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 k").unwrap().as_kelvin(),
        );
    }

    #[test]
    fn rankine_from_string() {
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123R").unwrap().as_rankine(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 R").unwrap().as_rankine(),
        );
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123 r").unwrap().as_rankine(),
        );
    }

    #[test]
    fn default_from_string() {
        assert_almost_equal(
            123.0,
            TemperatureBuilder::from_str("123").unwrap().as_celsius(),
        );

        assert_almost_equal(123.0, VolumeBuilder::from_str("123").unwrap().as_litres());
    }

    #[test]
    fn zero_from_string() {
        assert_eq!(0.0, TemperatureBuilder::from_str("").unwrap().as_celsius());
        assert_eq!(0.0, VolumeBuilder::from_str("").unwrap().as_litres());
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123cm3")
                .unwrap()
                .as_cubic_centimeters(),
        );
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123 cm3")
                .unwrap()
                .as_cubic_centimeters(),
        );
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123 CM3")
                .unwrap()
                .as_cubic_centimeters(),
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123ml").unwrap().as_milliliters(),
        );
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123 ml").unwrap().as_milliliters(),
        );
        assert_almost_equal(
            123.0,
            VolumeBuilder::from_str("123 ml").unwrap().as_milliliters(),
        );
    }

    #[test]
    fn pints_from_string() {
        assert_almost_equal(123.0, VolumeBuilder::from_str("123p").unwrap().as_pints());
        assert_almost_equal(123.0, VolumeBuilder::from_str("123 p").unwrap().as_pints());
        assert_almost_equal(123.0, VolumeBuilder::from_str("123 P").unwrap().as_pints());
    }
}
