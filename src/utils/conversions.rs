use measurements::{Temperature, Volume};

pub struct TemperatureBuilder;

impl TemperatureBuilder {
    pub fn from_str<S: Into<String>>(val: S) -> measurements::Temperature {
        let temp = val.into().to_uppercase();
        if temp.is_empty() {
            return Temperature::from_celsius(0.0);
        }

        let last = temp.chars().last().unwrap();
        let ret = match last {
            'F' => Temperature::from_fahrenheit(temp[..temp.len()-1].parse::<f64>().unwrap()),
            'C' => Temperature::from_celsius(temp[..temp.len()-1].parse::<f64>().unwrap()),
            'K' => Temperature::from_kelvin(temp[..temp.len()-1].parse::<f64>().unwrap()),
            'R' => Temperature::from_rankine(temp[..temp.len()-1].parse::<f64>().unwrap()),
            _ => Temperature::from_celsius(temp.parse::<f64>().unwrap())
        };
        ret
    }
}

pub struct VolumeBuilder;

impl VolumeBuilder {
    pub fn from_str<S: Into<String>>(val: S) -> measurements::Volume {
        let vol = val.into().to_lowercase();
        if vol.is_empty() {
            return Volume::from_litres(0.0);
        }

        if vol.len() > 3 {
            let last3 = &vol[vol.len() - 3..];
            let ret = match last3 {
                "cm3" => Ok(Volume::from_cubic_centimeters(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "ft3" => Ok(Volume::from_cubic_feet(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "yd3" => Ok(Volume::from_cubic_yards(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "in3" => Ok(Volume::from_cubic_inches(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "gal" => Ok(Volume::from_gallons(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "cup" => Ok(Volume::from_cups(vol[..vol.len()-3].parse::<f64>().unwrap())),
                "tsp" => Ok(Volume::from_teaspoons(vol[..vol.len()-3].parse::<f64>().unwrap())),
                _ => Err(last3)
            };

            if ret.is_ok() {
                return ret.unwrap();
            }
        }

        if vol.len() > 2 {
            let last2 = &vol[vol.len() - 2..];
            let ret = match last2 {
                "ml" => Ok(Volume::from_milliliters(vol[..vol.len()-2].parse::<f64>().unwrap())),
                "m3" => Ok(Volume::from_cubic_meters(vol[..vol.len()-2].parse::<f64>().unwrap())),
                "μl" => Ok(Volume::from_drops(vol[..vol.len()-2].parse::<f64>().unwrap())),
                "dr" => Ok(Volume::from_drams(vol[..vol.len()-2].parse::<f64>().unwrap())),
                _ => Err(last2)
            };

            if ret.is_ok() {
                return ret.unwrap();
            }
        }

        let last = vol.chars().last().unwrap();
        let ret = match last {
            'l' => Volume::from_litres(vol[..vol.len()-1].parse::<f64>().unwrap()),
            'p' => Volume::from_pints(vol[..vol.len()-1].parse::<f64>().unwrap()),
            'ʒ' => Volume::from_pints(vol[..vol.len()-1].parse::<f64>().unwrap()),
            _ => Volume::from_litres(vol.parse::<f64>().unwrap())
        };
        ret
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::conversions::TemperatureBuilder;
    use crate::utils::conversions::VolumeBuilder;

    #[test]
    fn fahrenheit_from_string() {
        assert_eq!(123.0, TemperatureBuilder::from_str("123F").as_fahrenheit().round());
    }

    #[test]
    fn celsius_from_string() {
        assert_eq!(123.0, TemperatureBuilder::from_str("123C").as_celsius().round());
    }

    #[test]
    fn kelvin_from_string() {
        assert_eq!(123.0, TemperatureBuilder::from_str("123K").as_kelvin().round());
    }

    #[test]
    fn rankine_from_string() {
        assert_eq!(123.0, TemperatureBuilder::from_str("123R").as_rankine().round());
    }

    #[test]
    fn default_from_string() {
        assert_eq!(123.0, TemperatureBuilder::from_str("123").as_celsius().round());
    }

    #[test]
    fn zero_from_string() {
        assert_eq!(0.0, TemperatureBuilder::from_str("").as_celsius().round());
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_eq!(123.0, VolumeBuilder::from_str("123cm3").as_cubic_centimeters().round());
    }

    #[test]
    fn milliliters_from_string() {
        assert_eq!(123.0, VolumeBuilder::from_str("123ml").as_milliliters().round());
    }

    #[test]
    fn pints_from_string() {
        assert_eq!(123.0, VolumeBuilder::from_str("123p").as_pints().round());
    }
}
