use measurements::Temperature;

pub struct TemperatureBuilder;

impl TemperatureBuilder {
    pub fn from_str<S: Into<String>>(val: S) -> measurements::Temperature {
        let temp = val.into();
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

#[cfg(test)]
mod tests {

    use crate::utils::conversions::TemperatureBuilder;

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
}
