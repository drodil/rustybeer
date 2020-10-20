use once_cell::sync::Lazy;
use serde::Deserialize;

/// Data for 12 oz ABV% to approx. calories
#[derive(Debug, Clone, Deserialize)]
pub struct ABVCalories {
    pub abv: f32,
    pub calories_low: f32,
    pub calories_high: f32,
}

// Raw inlined style data
static ABV_CALORIES_JSON: &str = include_str!("abv_calories.json");

/// All available ABV to calories.
///
/// Data will be loaded from JSON on the first use.
pub static ABV_CALORIES: Lazy<Vec<ABVCalories>> = Lazy::new(|| {
    serde_json::from_str(ABV_CALORIES_JSON).expect("ABV to calories data could not be deserialised")
});

/// Criteria for selecting a ABV to calories.
///
/// If an attribute is `None`, it is ignored.
#[derive(Debug, Clone, Default)]
pub struct Criteria {
    pub abv: Option<f32>,
}

impl Criteria {
    /// To check whether the given ABV is within range
    pub fn matches(&self, cal: &ABVCalories) -> bool {
        if let Some(abv) = self.abv {
            if (abv - 0.5) >= cal.abv {
                return false;
            }
        }

        true
    }
}
