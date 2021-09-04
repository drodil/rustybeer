/// Yeast list curated from https://www.brewersfriend.com/yeast/
use measurements::temperature::Temperature;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use crate::conversions::TemperatureParser;
use crate::strings::contains_case_insensitive;

#[derive(Debug)]
pub enum Level {
    Low,
    MedLow,
    Medium,
    MedHigh,
    High,
    VeryHigh,
}

#[derive(Debug, Default, Deserialize)]
pub struct Yeast {
    pub company: String,
    pub name: String,
    pub id: Option<String>,
    pub min_attenuation: Option<u8>,
    pub max_attenuation: Option<u8>,
    #[serde(deserialize_with = "level_from_str")]
    pub attenuation_level: Option<Level>,
    #[serde(deserialize_with = "level_from_str")]
    pub flocculation: Option<Level>,
    #[serde(deserialize_with = "temp_from_str")]
    pub min_temp: Option<Temperature>,
    #[serde(deserialize_with = "temp_from_str")]
    pub max_temp: Option<Temperature>,
    pub alc_tolerance: Option<u8>,
    #[serde(deserialize_with = "level_from_str")]
    pub alc_tolerance_level: Option<Level>,
}

fn level_from_str<'de, D>(deserializer: D) -> Result<Option<Level>, D::Error>
where
    D: Deserializer<'de>,
{
    let m = <&str>::deserialize(deserializer);
    match m {
        Ok(s) => match s {
            "Low" => Ok(Some(Level::Low)),
            "Med-Low" => Ok(Some(Level::MedLow)),
            "Medium" => Ok(Some(Level::Medium)),
            "Med-High" => Ok(Some(Level::MedHigh)),
            "High" => Ok(Some(Level::High)),
            "Very High" => Ok(Some(Level::VeryHigh)),
            _ => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

fn temp_from_str<'de, D>(deserializer: D) -> Result<Option<Temperature>, D::Error>
where
    D: Deserializer<'de>,
{
    let t = u8::deserialize(deserializer);
    match t {
        Ok(v) => Ok(Some(Temperature::from_fahrenheit(v as f64))),
        Err(_) => Ok(None),
    }
}

static YEASTS_JSON: &str = include_str!("json/yeasts.json");

/// All available yeasts.
///
/// Data will be loaded from JSON on the first use.
pub static YEASTS: Lazy<Vec<Yeast>> =
    Lazy::new(|| serde_json::from_str(YEASTS_JSON).expect("yeasts data could not be deserialised"));


/// Criteria for selecting a yeast.
///
/// If an attribute is `None`, it is ignored.
#[derive(Debug, Clone, Default)]
pub struct Criteria {
    pub company: Option<String>,
    pub name: Option<String>,
    pub attenuation: Option<u8>,
    pub temperature: Option<String>,
}

impl Criteria {
    /// Whether the given yeast matches **all** criteria that are `Some`.
    pub fn matches(&self, yeast: &Yeast) -> bool {
        if let Some(company) = &self.company {
            if !contains_case_insensitive(&yeast.company, &company) {
                return false;
            }
        }

        if let Some(name) = &self.name {
            if !contains_case_insensitive(&yeast.name, &name) {
                return false;
            }
        }

        if let Some(attenuation) = self.attenuation {
            if attenuation < yeast.min_attenuation.unwrap() || attenuation > yeast.max_attenuation.unwrap() {
                return false;
            }
        }

        if let Some(temperature) = &self.temperature {
            if let Ok(temp) = TemperatureParser::parse(temperature) {
                if temp < yeast.min_temp.unwrap() || temp > yeast.max_temp.unwrap() {
                    return false;
                }
            }
        }

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_YEAST: Lazy<Yeast> = Lazy::new(|| Yeast {
        name: "I am yeast 66".to_owned(),
        company: "Yeast Corp.".to_owned(),
        min_attenuation: Some(6),
        max_attenuation: Some(14),
        min_temp: Some(Temperature::from_celsius(14.0)),
        max_temp: Some(Temperature::from_celsius(25.0)),
        alc_tolerance: None,
        alc_tolerance_level: None,
        attenuation_level: None,
        id: None,
        flocculation: None,
    });

    #[test]
    fn no_criteria_matches() {
        let criteria = Criteria::default();
        assert!(criteria.matches(&TEST_YEAST));
    }

    #[test]
    fn criteria_matches_inclusive() {
        let mut criteria = Criteria::default();
        assert!(criteria.matches(&TEST_YEAST));

        // Out of range values fails
        criteria.attenuation = Some(1);
        assert!(!criteria.matches(&TEST_YEAST));
        criteria.attenuation = Some(20);
        assert!(!criteria.matches(&TEST_YEAST));

        // Inclusive values match
        criteria.attenuation = Some(7);
        assert!(criteria.matches(&TEST_YEAST));
        criteria.temperature = Some("20C".to_owned());
        assert!(criteria.matches(&TEST_YEAST));
        criteria.company = Some("yeast".to_owned());
        assert!(criteria.matches(&TEST_YEAST));
        criteria.name = Some("66".to_owned());
        assert!(criteria.matches(&TEST_YEAST));
    }

}
