/// Hops list curated from https://www.brewersfriend.com/2010/02/27/hops-alpha-acid-table-2009/
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Hop {
    #[serde(alias = "hop")]
    pub name: String,

    #[serde(alias = "alpha_acids", deserialize_with = "percentage_to_float")]
    pub alpha_acid: f64,
}

const HOPS_JSON: &str = include_str!("json/hops.json");

fn percentage_to_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(f64::deserialize(deserializer)? / 100.)
}

pub static HOPS: Lazy<Vec<Hop>> = Lazy::new(|| serde_json::from_str(HOPS_JSON).unwrap());

#[cfg(test)]
pub mod test {
    use super::{Hop, HOPS};

    #[test]
    fn centennial() {
        assert_eq!(
            Some(&Hop {
                name: "Centennial".to_string(),
                alpha_acid: 0.085
            }),
            HOPS.iter().find(|&h| h.name == "Centennial")
        )
    }
}
