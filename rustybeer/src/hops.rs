use crate::strings::contains_case_insensitive;
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

/// Criteria for selecting a hop.
///
/// If an attribute is `None`, it is ignored.
#[derive(Debug, Clone, Default)]
pub struct Criteria {
    pub name: Option<String>,
    pub country: Option<String>,
    pub alpha_acid: Option<f64>,
    pub beta_acid: Option<f64>,
    pub purpose: Option<String>,
    pub substituted: Option<String>,
}

impl Criteria {
    /// Whether the given hop matches **all** criteria that are `Some`.
    pub fn matches(&self, hop: &Hop) -> bool {
        if let Some(name) = &self.name {
            if !contains_case_insensitive(&hop.name, &name) {
                return false;
            }
        }
        if let Some(country) = &self.country {
            if !contains_case_insensitive(&hop.country, &country) {
                return false;
            }
        }
        if let Some(alpha_acid) = self.alpha_acid {
            if alpha_acid < hop.alpha_acid_min || alpha_acid > hop.alpha_acid_max {
                return false;
            }
        }
        if let Some(beta_acid) = self.beta_acid {
            if beta_acid < hop.beta_acid_min || beta_acid > hop.beta_acid_max {
                return false;
            }
        }
        if let Some(purpose) = &self.purpose {
            if !hop.purpose.contains(&purpose) {
                return false;
            }
        }
        if let Some(substituted) = &self.substituted {
            if !hop.substitutions.contains(&substituted) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn centennial() {
        assert_relative_eq!(
            0.095,
            HOPS.iter()
                .find(|&h| h.name == "Centennial")
                .unwrap()
                .alpha_acid_min
        );
    }

    static TEST_HOP: Lazy<Hop> = Lazy::new(|| Hop {
        name: "test hop".to_owned(),
        alpha_acid_min: 4.0,
        alpha_acid_max: 5.3,
        beta_acid_min: 5.0,
        beta_acid_max: 6.5,
        purpose: vec!["Aroma".to_string()],
        country: "US".to_owned(),
        description: "Test".to_owned(),
        substitutions: vec!["Cascade".to_string()],
    });

    #[test]
    fn no_criteria_matches() {
        let criteria = Criteria::default();
        assert!(criteria.matches(&TEST_HOP));
    }

    #[test]
    fn criteria_matches_inclusive() {
        let mut criteria = Criteria::default();

        // Out of range values fails
        criteria.alpha_acid = Some(-0.1);
        assert!(!criteria.matches(&TEST_HOP));
        criteria.alpha_acid = Some(60.1);
        assert!(!criteria.matches(&TEST_HOP));

        // Inclusive values match
        criteria.alpha_acid = Some(5.0);
        assert!(criteria.matches(&TEST_HOP));
        criteria.beta_acid = Some(5.5);
        assert!(criteria.matches(&TEST_HOP));
        criteria.purpose = Some("Aroma".to_string());
        assert!(criteria.matches(&TEST_HOP));
        criteria.substituted = Some("Cascade".to_string());
        assert!(criteria.matches(&TEST_HOP));
        criteria.name = Some("hop".to_owned());
        assert!(criteria.matches(&TEST_HOP));
        criteria.country = Some("us".to_owned());
        assert!(criteria.matches(&TEST_HOP));
    }
}
