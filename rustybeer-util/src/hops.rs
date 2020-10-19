/// Hops list curated from https://www.brewersfriend.com/2010/02/27/hops-alpha-acid-table-2009/
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Hop {
    pub name: String,
    #[serde(alias = "aa_min", deserialize_with = "percentage_to_float")]
    pub alpha_acid_min: f64,
    #[serde(alias = "aa_max", deserialize_with = "percentage_to_float")]
    pub alpha_acid_max: f64,
    #[serde(alias = "ba_min", deserialize_with = "percentage_to_float")]
    pub beta_acid_min: f64,
    #[serde(alias = "ba_max", deserialize_with = "percentage_to_float")]
    pub beta_acid_max: f64,
    pub purpose: Vec<String>,
    pub country: String,
    pub description: String,
    pub substitutions: Vec<String>,
}

const HOPS_JSON: &str = include_str!("json/hops.json");

fn percentage_to_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(f64::deserialize(deserializer)? / 100.)
}

impl Hop {
    /// Returns average alpha acid for the hop.
    pub fn average_alpha_acid(&self) -> f64 {
        (self.alpha_acid_min + self.alpha_acid_max) / 2.0
    }
}

pub static HOPS: Lazy<Vec<Hop>> = Lazy::new(|| serde_json::from_str(HOPS_JSON).unwrap());

#[cfg(test)]
pub mod test {
    use super::{Hop, HOPS};

    #[test]
    fn centennial() {
        assert_eq!(
            0.095,
            HOPS.iter()
                .find(|&h| h.name == "Centennial")
                .unwrap()
                .alpha_acid_min
        );
    }
}
