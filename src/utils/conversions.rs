use std::num::ParseFloatError;

use measurements::{Temperature, Volume};

pub struct TemperatureBuilder;

impl TemperatureBuilder {
    pub fn from_str<S: Into<String>>(val: S) -> Result<measurements::Temperature, ParseFloatError> {
        let temp = val.into().to_uppercase();
        if temp.is_empty() {
            return Ok(Temperature::from_celsius(0.0));
        }

        let last = temp.chars().last().unwrap();
        let ret = match last {
            'F' => Temperature::from_fahrenheit(temp[..temp.len() - 1].parse::<f64>()?),
            'C' => Temperature::from_celsius(temp[..temp.len() - 1].parse::<f64>()?),
            'K' => Temperature::from_kelvin(temp[..temp.len() - 1].parse::<f64>()?),
            'R' => Temperature::from_rankine(temp[..temp.len() - 1].parse::<f64>()?),
            _ => Temperature::from_celsius(temp.parse::<f64>()?),
        };
        Ok(ret)
    }
}

pub struct VolumeBuilder;

impl VolumeBuilder {
    pub fn from_str<S: Into<String>>(val: S) -> Result<measurements::Volume, ParseFloatError> {
        let vol = val.into().to_lowercase();
        if vol.is_empty() {
            return Ok(Volume::from_litres(0.0));
        }

        match vol.find(':') {
            Some(count) => {
                let (temperature, mut temperature_form) = vol.split_at(count);
                temperature_form = &temperature_form[1..];
                let len = temperature_form.len();
                if len == 3 {
                    return Ok(match temperature_form {
                        "cm3" => Volume::from_cubic_centimeters(temperature.parse::<f64>()?),
                        "ft3" => Volume::from_cubic_feet(temperature.parse::<f64>()?),
                        "yd3" => Volume::from_cubic_yards(temperature.parse::<f64>()?),
                        "in3" => Volume::from_cubic_inches(temperature.parse::<f64>()?),
                        "gal" => Volume::from_gallons(temperature.parse::<f64>()?),
                        "cup" => Volume::from_cups(temperature.parse::<f64>()?),
                        "tsp" => Volume::from_teaspoons(temperature.parse::<f64>()?),
                        _ => Volume::from_cubic_centimeters(temperature.parse::<f64>()?),
                    });
                } else if len == 2 {
                    return Ok(match temperature_form {
                        "ml" => Volume::from_milliliters(temperature.parse::<f64>()?),
                        "m3" => Volume::from_cubic_meters(temperature.parse::<f64>()?),
                        "μl" => Volume::from_drops(temperature.parse::<f64>()?),
                        "dr" => Volume::from_drams(temperature.parse::<f64>()?),
                        _ => Volume::from_milliliters(temperature.parse::<f64>()?),
                    });
                } else {
                    return Ok(match temperature_form {
                        "l" => Volume::from_litres(temperature.parse::<f64>()?),
                        "p" => Volume::from_pints(temperature.parse::<f64>()?),
                        "ʒ" => Volume::from_pints(temperature.parse::<f64>()?),
                        _ => Volume::from_litres(vol.parse::<f64>()?),
                    });
                }
            }
            None => panic!("When parsing volumes, a ':' is required between "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TemperatureBuilder;
    use super::VolumeBuilder;

    #[test]
    fn fahrenheit_from_string() {
        assert_eq!(
            123.0,
            TemperatureBuilder::from_str("123F")
                .unwrap()
                .as_fahrenheit()
                .round()
        );
    }

    #[test]
    fn celsius_from_string() {
        assert_eq!(
            123.0,
            TemperatureBuilder::from_str("123C")
                .unwrap()
                .as_celsius()
                .round()
        );
    }

    #[test]
    fn kelvin_from_string() {
        assert_eq!(
            123.0,
            TemperatureBuilder::from_str("123K")
                .unwrap()
                .as_kelvin()
                .round()
        );
    }

    #[test]
    fn rankine_from_string() {
        assert_eq!(
            123.0,
            TemperatureBuilder::from_str("123R")
                .unwrap()
                .as_rankine()
                .round()
        );
    }

    #[test]
    fn default_from_string() {
        assert_eq!(
            123.0,
            TemperatureBuilder::from_str("123")
                .unwrap()
                .as_celsius()
                .round()
        );
    }

    #[test]
    fn zero_from_string() {
        assert_eq!(
            0.0,
            TemperatureBuilder::from_str("")
                .unwrap()
                .as_celsius()
                .round()
        );
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_eq!(
            123.0,
            VolumeBuilder::from_str("123:cm3")
                .unwrap()
                .as_cubic_centimeters()
                .round()
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_eq!(
            123.0,
            VolumeBuilder::from_str("123:ml")
                .unwrap()
                .as_milliliters()
                .round()
        );
    }

    #[test]
    fn pints_from_string() {
        assert_eq!(
            123.0,
            VolumeBuilder::from_str("123:p").unwrap().as_pints().round()
        );
    }
}
