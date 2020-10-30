use csv::ReaderBuilder;
use measurements::temperature::Temperature;
//use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::error::Error;

#[derive(Debug)]
pub enum Level {
    Low,
    MedLow,
    Medium,
    MedHigh,
    High,
    VeryHigh,
}

#[derive(Debug)]
pub enum Tolerance {
    Level(Level),
    Percentage(u8),
}

impl Default for Tolerance {
    fn default() -> Self {
        Tolerance::Percentage(0)
    }
}

#[derive(Debug)]
struct MinMaxTemperature {
    min: Temperature,
    max: Temperature,
}

#[derive(Debug)]
struct MinMaxU8 {
    min: u8,
    max: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Attenuation {
    Level(Level),
    Percentages(MinMaxU8),
}

#[derive(Debug, Default, Deserialize)]
pub struct Yeast {
    company: String,
    name: String,
    id: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "att_from_str")]
    attenuation: Option<Attenuation>,
    #[serde(default)]
    #[serde(deserialize_with = "level_from_str")]
    flocculation: Option<Level>,
    #[serde(default)]
    #[serde(deserialize_with = "temp_from_str")]
    opt_temperature: Option<MinMaxTemperature>,
    #[serde(default)]
    #[serde(deserialize_with = "tolerance_from_str")]
    alc_tolerance: Option<Tolerance>,
}

fn att_from_str<'de, D>(_deserializer: D) -> Result<Option<Attenuation>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

fn level_from_str<'de, D>(deserializer: D) -> Result<Option<Level>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    match s {
        "Low" => Ok(Some(Level::Low)),
        "Med-Low" => Ok(Some(Level::MedLow)),
        "Medium" => Ok(Some(Level::Medium)),
        "Med-High" => Ok(Some(Level::MedHigh)),
        "High" => Ok(Some(Level::High)),
        "Very High" => Ok(Some(Level::VeryHigh)),
        _ => Ok(None),
    }
}

fn temp_from_str<'de, D>(_deserializer: D) -> Result<Option<MinMaxTemperature>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

fn tolerance_from_str<'de, D>(_deserializer: D) -> Result<Option<Tolerance>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(None)
}

use std::path::Path;

pub fn read_yeasts(path: &Path) -> Result<Vec<Yeast>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    let records = rdr
        .deserialize()
        .collect::<Result<Vec<Yeast>, csv::Error>>()?;
    Ok(records)
}
