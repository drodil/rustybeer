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
                let (temperature, temperature_form) = vol.split_at(count);
                println!("{}", temperature);
                println!("{}", temperature_form);
            }
            None => {}
        }

        if vol.len() > 3 {
            let last3 = &vol[vol.len() - 3..];
            println!("{}", last3);
            println!("{}", &vol[..vol.len() - 3]);
            let ret = match last3 {
                "cm3" => Volume::from_cubic_centimeters(vol[..vol.len() - 3].parse::<f64>()?),
                "ft3" => Volume::from_cubic_feet(vol[..vol.len() - 3].parse::<f64>()?),
                "yd3" => Volume::from_cubic_yards(vol[..vol.len() - 3].parse::<f64>()?),
                "in3" => Volume::from_cubic_inches(vol[..vol.len() - 3].parse::<f64>()?),
                "gal" => Volume::from_gallons(vol[..vol.len() - 3].parse::<f64>()?),
                "cup" => Volume::from_cups(vol[..vol.len() - 3].parse::<f64>()?),
                "tsp" => Volume::from_teaspoons(vol[..vol.len() - 3].parse::<f64>()?),
                _ => Volume::from_cubic_centimeters(vol[..vol.len() - 3].parse::<f64>()?),
            };
            Ok(ret)
        } else if vol.len() > 2 {
            let last2 = &vol[vol.len() - 2..];
            let ret = match last2 {
                "ml" => Volume::from_milliliters(vol[..vol.len() - 2].parse::<f64>()?),
                "m3" => Volume::from_cubic_meters(vol[..vol.len() - 2].parse::<f64>()?),
                "μl" => Volume::from_drops(vol[..vol.len() - 2].parse::<f64>()?),
                "dr" => Volume::from_drams(vol[..vol.len() - 2].parse::<f64>()?),
                _ => Volume::from_milliliters(vol[..vol.len() - 3].parse::<f64>()?),
            };
            Ok(ret)
        } else {
            let last = vol.chars().last().unwrap();
            let ret = match last {
                'l' => Volume::from_litres(vol[..vol.len() - 1].parse::<f64>()?),
                'p' => Volume::from_pints(vol[..vol.len() - 1].parse::<f64>()?),
                'ʒ' => Volume::from_pints(vol[..vol.len() - 1].parse::<f64>()?),
                _ => Volume::from_litres(vol.parse::<f64>()?),
            };
            Ok(ret)
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
            VolumeBuilder::from_str("123cm3")
                .unwrap()
                .as_cubic_centimeters()
                .round()
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_eq!(
            123.0,
            VolumeBuilder::from_str("123ml")
                .unwrap()
                .as_milliliters()
                .round()
        );
    }

    #[test]
    fn pints_from_string() {
        assert_eq!(
            123.0,
            VolumeBuilder::from_str("123p").unwrap().as_pints().round()
        );
    }
}
