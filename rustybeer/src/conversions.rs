use measurements::{Energy, Mass, Temperature, Volume};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::ParseFloatError;

/// Used to build new measurements::Energy structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct EnergyParser;

impl EnergyParser {
    /// Creates measurements::Energy from string
    ///
    /// Tries to figure out the energy unit from the string. If the string value is plain
    /// number, it will be considered as kilocalories. Also empty strings are considered as
    /// zero kilocalories in Energy.
    pub fn parse(val: &str) -> Result<Energy, ParseFloatError> {
        if val.is_empty() {
            return Ok(Energy::from_kcalories(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Zμ]{1,4})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_uppercase().as_str() {
                    "KCAL" => Energy::from_kcalories(float_val.parse::<f64>()?),
                    "BTU" => Energy::from_btu(float_val.parse::<f64>()?),
                    "EV" => Energy::from_e_v(float_val.parse::<f64>()?),
                    "WH" => Energy::from_watt_hours(float_val.parse::<f64>()?),
                    "KWH" => Energy::from_kilowatt_hours(float_val.parse::<f64>()?),
                    "J" => Energy::from_joules(float_val.parse::<f64>()?),
                    _ => Energy::from_kcalories(float_val.parse::<f64>()?),
                },
            );
        }

        Ok(Energy::from_kcalories(val.parse::<f64>()?))
    }
}

/// Convertes energy to map for displaying
pub fn energy_to_map(e: Energy) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    map.insert("J".to_owned(), e.as_joules());
    map.insert("Kcal".to_owned(), e.as_kcalories());
    map.insert("Btu".to_owned(), e.as_btu());
    map.insert("eV".to_owned(), e.as_e_v());
    map.insert("Wh".to_owned(), e.as_watt_hours());
    map.insert("KWh".to_owned(), e.as_kilowatt_hours());
    map
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

/// Converts temperature unit to map for displaying
pub fn temp_to_map(t: Temperature) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    map.insert("celsius".to_owned(), t.as_celsius());
    map.insert("fahrenheit".to_owned(), t.as_fahrenheit());
    map.insert("kelvin".to_owned(), t.as_kelvin());
    map.insert("rankine".to_owned(), t.as_rankine());
    map
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

/// Relative density struct.
///
/// Also known as specific gravity which can be presented in different units
/// like plato and brix.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RelativeDensity {
    sg: f64,
}

impl RelativeDensity {
    pub fn from_specific_gravity(sg: f64) -> RelativeDensity {
        RelativeDensity { sg }
    }

    pub fn from_plato(plato: f64) -> RelativeDensity {
        RelativeDensity::from_specific_gravity(1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1))))
    }

    pub fn from_brix(brix: f64) -> RelativeDensity {
        RelativeDensity::from_specific_gravity((brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.0)
    }

    pub fn as_specific_gravity(&self) -> f64 {
        self.sg
    }

    pub fn as_plato(&self) -> f64 {
        (-1.0 * 616.868) + (1111.14 * self.sg) - (630.272 * self.sg.powi(2))
            + (135.997 * self.sg.powi(3))
    }

    pub fn as_brix(&self) -> f64 {
        ((182.4601 * self.sg - 775.6821) * self.sg + 1262.7794) * self.sg - 669.5622
    }
}

/// Used to build new conversions::RelativeDensity structs.
pub struct RelativeDensityParser;

impl RelativeDensityParser {
    /// Creates conversions::RelativeDensity from string
    ///
    /// Tries to figure out the volume unit from the string. If the string value is plain
    /// number, it will be considered as specific gravity. Also empty strings are considered as
    /// zero sg.
    pub fn parse(val: &str) -> Result<RelativeDensity, ParseFloatError> {
        if val.is_empty() {
            return Ok(RelativeDensity::from_specific_gravity(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([°PpBbxX]{1,3})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_lowercase().as_str() {
                    "p" => RelativeDensity::from_plato(float_val.parse::<f64>()?),
                    "bx" => RelativeDensity::from_brix(float_val.parse::<f64>()?),
                    "°p" => RelativeDensity::from_plato(float_val.parse::<f64>()?),
                    "°bx" => RelativeDensity::from_brix(float_val.parse::<f64>()?),
                    _ => RelativeDensity::from_specific_gravity(val.parse::<f64>()?),
                },
            );
        }

        Ok(RelativeDensity::from_specific_gravity(val.parse::<f64>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EnergyParser, MassParser, RelativeDensity, RelativeDensityParser, TemperatureParser,
        VolumeParser,
    };
    use approx::assert_relative_eq;

    #[test]
    fn default_from_string() {
        assert_relative_eq!(123.0, EnergyParser::parse("123").unwrap().as_kcalories(),);
        assert_relative_eq!(123.0, MassParser::parse("123").unwrap().as_grams(),);
        assert_relative_eq!(123.0, TemperatureParser::parse("123").unwrap().as_celsius(),);
        assert_relative_eq!(123.0, VolumeParser::parse("123").unwrap().as_litres(),);
    }

    #[test]
    fn zero_from_string() {
        assert_relative_eq!(0., EnergyParser::parse("").unwrap().as_kcalories());
        assert_relative_eq!(0., MassParser::parse("").unwrap().as_grams());
        assert_relative_eq!(0., TemperatureParser::parse("").unwrap().as_celsius(),);
        assert_relative_eq!(0., VolumeParser::parse("").unwrap().as_litres());
    }

    // Energy
    #[test]
    fn kcalories_from_string() {
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123kcal").unwrap().as_kcalories(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 kcal").unwrap().as_kcalories(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123KCAL").unwrap().as_kcalories(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 KCAL").unwrap().as_kcalories(),
        );
    }

    #[test]
    fn btus_from_string() {
        assert_relative_eq!(123.0, EnergyParser::parse("123btu").unwrap().as_btu(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 btu").unwrap().as_btu(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123BTU").unwrap().as_btu(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 BTU").unwrap().as_btu(),);
    }

    #[test]
    fn electronvolts_from_string() {
        assert_relative_eq!(123.0, EnergyParser::parse("123ev").unwrap().as_e_v(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 ev").unwrap().as_e_v(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123EV").unwrap().as_e_v(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 EV").unwrap().as_e_v(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123eV").unwrap().as_e_v(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 eV").unwrap().as_e_v(),);
    }

    #[test]
    fn watthours_from_string() {
        assert_relative_eq!(123.0, EnergyParser::parse("123wh").unwrap().as_watt_hours(),);
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 wh").unwrap().as_watt_hours(),
        );
        assert_relative_eq!(123.0, EnergyParser::parse("123WH").unwrap().as_watt_hours(),);
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 WH").unwrap().as_watt_hours(),
        );
        assert_relative_eq!(123.0, EnergyParser::parse("123Wh").unwrap().as_watt_hours(),);
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 Wh").unwrap().as_watt_hours(),
        );
    }

    #[test]
    fn kilowatthours_from_string() {
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123kwh").unwrap().as_kilowatt_hours(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 kwh").unwrap().as_kilowatt_hours(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123KWH").unwrap().as_kilowatt_hours(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 KWH").unwrap().as_kilowatt_hours(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123kWh").unwrap().as_kilowatt_hours(),
        );
        assert_relative_eq!(
            123.0,
            EnergyParser::parse("123 kWh").unwrap().as_kilowatt_hours(),
        );
    }

    #[test]
    fn joules_from_string() {
        assert_relative_eq!(123.0, EnergyParser::parse("123j").unwrap().as_joules(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 j").unwrap().as_joules(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123J").unwrap().as_joules(),);
        assert_relative_eq!(123.0, EnergyParser::parse("123 J").unwrap().as_joules(),);
    }

    // Mass
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

    // Temperature
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

    // Volume
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
    fn relative_density_conversions() {
        let rd = RelativeDensity::from_specific_gravity(1.092);
        assert_relative_eq!(1.092, rd.as_specific_gravity());
        assert_relative_eq!(22.01, f64::trunc(rd.as_plato() * 100.0) / 100.0);
        assert_relative_eq!(22.01, f64::trunc(rd.as_brix() * 100.0) / 100.0);
    }

    #[test]
    fn relative_density_from_string() {
        assert_eq!(
            22,
            RelativeDensityParser::parse("22°P").unwrap().as_plato() as i32
        );
        assert_eq!(
            22,
            RelativeDensityParser::parse("22 P").unwrap().as_plato() as i32
        );
        assert_eq!(
            22,
            RelativeDensityParser::parse("22 °bX").unwrap().as_brix() as i32
        );
        assert_eq!(
            22,
            RelativeDensityParser::parse("22bx").unwrap().as_brix() as i32
        );
        assert_relative_eq!(
            1.092,
            RelativeDensityParser::parse("1.092")
                .unwrap()
                .as_specific_gravity()
        );
    }
}
