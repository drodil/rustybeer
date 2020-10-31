/// Yeast list curated from https://www.brewersfriend.com/yeast/
use measurements::temperature::Temperature;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

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
