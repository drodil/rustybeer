use measurements::{Mass, Temperature, Volume};
use regex::Regex;
use std::num::ParseFloatError;

/// Used to build new Temperature structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct TemperatureParser;

impl TemperatureParser {
    /// Creates measurements::Temperature from string
    ///
    /// Tries to figure out the temperature unit from the string. If the string value is plain
    /// number, it will be considered as Celsius. Also empty strings are considered as
    /// zero Celsius in Temperature.
    pub fn parse(val: &str) -> Result<Temperature, ParseFloatError> {
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
pub struct VolumeParser;

impl VolumeParser {
    /// Creates measurements::Volume from string
    ///
    /// Tries to figure out the volume unit from the string. If the string value is plain
    /// number, it will be considered as litres. Also empty strings are considered as
    /// zero litres in Volume.
    pub fn parse(val: &str) -> Result<Volume, ParseFloatError> {
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
pub struct MassParser;

impl MassParser {
    /// Creates measurements::Mass from string
    ///
    /// Tries to figure out the mass unit from the string. If the string value is plain
    /// number, it will be considered as grams. Also empty strings are considered as
    /// zero grams in Mass.
    pub fn parse(val: &str) -> Result<Mass, ParseFloatError> {
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
    use super::{MassParser, TemperatureParser, VolumeParser};
    use approx::assert_relative_eq;

    #[test]
    fn fahrenheit_from_string() {
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123F").unwrap().as_fahrenheit(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 F").unwrap().as_fahrenheit(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 f").unwrap().as_fahrenheit(),
        );
    }

    #[test]
    fn celsius_from_string() {
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123C").unwrap().as_celsius(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 C").unwrap().as_celsius(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 c").unwrap().as_celsius(),
        );
    }

    #[test]
    fn kelvin_from_string() {
        assert_relative_eq!(123.0, TemperatureParser::parse("123K").unwrap().as_kelvin(),);
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 K").unwrap().as_kelvin(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 k").unwrap().as_kelvin(),
        );
    }

    #[test]
    fn rankine_from_string() {
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123R").unwrap().as_rankine(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 R").unwrap().as_rankine(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureParser::parse("123 r").unwrap().as_rankine(),
        );
    }

    #[test]
    fn default_from_string() {
        assert_relative_eq!(123.0, TemperatureParser::parse("123").unwrap().as_celsius(),);

        assert_relative_eq!(123.0, VolumeParser::parse("123").unwrap().as_litres(),);
        assert_relative_eq!(123.0, MassParser::parse("123").unwrap().as_grams(),);
    }

    #[test]
    fn zero_from_string() {
        assert_relative_eq!(0., TemperatureParser::parse("").unwrap().as_celsius(),);
        assert_relative_eq!(0., VolumeParser::parse("").unwrap().as_litres());
        assert_relative_eq!(0., MassParser::parse("").unwrap().as_grams());
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123cm3")
                .unwrap()
                .as_cubic_centimeters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123 cm3")
                .unwrap()
                .as_cubic_centimeters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123 CM3")
                .unwrap()
                .as_cubic_centimeters(),
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123ml").unwrap().as_milliliters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123 ml").unwrap().as_milliliters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeParser::parse("123 ml").unwrap().as_milliliters(),
        );
    }

    #[test]
    fn pints_from_string() {
        assert_relative_eq!(123.0, VolumeParser::parse("123p").unwrap().as_pints(),);
        assert_relative_eq!(123.0, VolumeParser::parse("123 p").unwrap().as_pints(),);
        assert_relative_eq!(123.0, VolumeParser::parse("123 P").unwrap().as_pints(),);
    }

    #[test]
    fn micrograms_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123ug").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassParser::parse("123 ug").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassParser::parse("123μg").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassParser::parse("123 μg").unwrap().as_micrograms(),);
    }

    #[test]
    fn milligrams_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123mg").unwrap().as_milligrams(),);
        assert_relative_eq!(123.0, MassParser::parse("123 mg").unwrap().as_milligrams(),);
    }

    #[test]
    fn carats_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123ct").unwrap().as_carats(),);
        assert_relative_eq!(123.0, MassParser::parse("123 ct").unwrap().as_carats(),);
    }

    #[test]
    fn grams_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123g").unwrap().as_grams(),);
        assert_relative_eq!(123.0, MassParser::parse("123 g").unwrap().as_grams(),);
    }

    #[test]
    fn kilograms_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123kg").unwrap().as_kilograms(),);
        assert_relative_eq!(123.0, MassParser::parse("123 kg").unwrap().as_kilograms(),);
    }

    #[test]
    fn tonnes_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123T").unwrap().as_tonnes(),);
        assert_relative_eq!(123.0, MassParser::parse("123 T").unwrap().as_tonnes(),);
    }

    #[test]
    fn grains_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123gr").unwrap().as_grains(),);
        assert_relative_eq!(123.0, MassParser::parse("123 gr").unwrap().as_grains(),);
    }

    #[test]
    fn pennyweights_from_string() {
        assert_relative_eq!(
            123.0,
            MassParser::parse("123dwt").unwrap().as_pennyweights(),
        );
        assert_relative_eq!(
            123.0,
            MassParser::parse("123 dwt").unwrap().as_pennyweights(),
        );
    }

    #[test]
    fn ounces_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123oz").unwrap().as_ounces(),);
        assert_relative_eq!(123.0, MassParser::parse("123 oz").unwrap().as_ounces(),);
    }

    #[test]
    fn pounds_from_string() {
        assert_relative_eq!(123.0, MassParser::parse("123lbs").unwrap().as_pounds(),);
        assert_relative_eq!(123.0, MassParser::parse("123 lbs").unwrap().as_pounds(),);
    }
}
