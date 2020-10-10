use once_cell::sync::Lazy;
use serde::Deserialize;

/// Data about a particular style of beer.
#[derive(Debug, Clone, Deserialize)]
pub struct BeerStyle {
    pub name: String,
    pub original_gravity_min: f32,
    pub original_gravity_max: f32,
    pub final_gravity_min: f32,
    pub final_gravity_max: f32,
    pub abv_min: f32,
    pub abv_max: f32,
    pub ibu_min: u8,
    pub ibu_max: u8,
    pub color_srm_min: f32,
    pub color_srm_max: f32,
    pub description: String,
}

// Raw inlined style data
static BEER_STYLES_JSON: &str = include_str!("json/beer_styles.json");

/// All available beer styles.
///
/// Data will be loaded from JSON on the first use.
pub static BEER_STYLES: Lazy<Vec<BeerStyle>> = Lazy::new(|| {
    serde_json::from_str(BEER_STYLES_JSON).expect("beer styles data could not be deserialised")
});

/// Criteria for selecting a beer style.
///
/// If an attribute is `None`, it is ignored.
#[derive(Debug, Clone, Default)]
pub struct Criteria {
    pub og: Option<f32>,
    pub fg: Option<f32>,
    pub abv: Option<f32>,
    pub ibu: Option<u8>,
    pub srm: Option<f32>,
}

impl Criteria {
    /// Whether the given beer style matches **all** criteria that are `Some`.
    pub fn matches(&self, style: &BeerStyle) -> bool {
        if let Some(og) = self.og {
            if og < style.original_gravity_min || og > style.original_gravity_max {
                return false;
            }
        }
        if let Some(fg) = self.fg {
            if fg < style.final_gravity_min || fg > style.final_gravity_max {
                return false;
            }
        }
        if let Some(abv) = self.abv {
            if abv < style.abv_min || abv > style.abv_max {
                return false;
            }
        }
        if let Some(ibu) = self.ibu {
            if ibu < style.ibu_min || ibu > style.ibu_max {
                return false;
            }
        }
        if let Some(srm) = self.srm {
            if srm < style.color_srm_min || srm > style.color_srm_max {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_BEER_STYLE: Lazy<BeerStyle> = Lazy::new(|| BeerStyle {
        name: "test beer style".to_owned(),
        original_gravity_min: 0.0,
        original_gravity_max: 1.0,
        final_gravity_min: -2.3,
        final_gravity_max: 4.8,
        abv_min: 0.0,
        abv_max: 0.0,
        ibu_min: 8,
        ibu_max: 16,
        color_srm_min: -1234.5,
        color_srm_max: 1234.5,
        description: String::from("Test"),
    });

    #[test]
    fn no_criteria_matches() {
        let criteria = Criteria::default();
        assert!(criteria.matches(&TEST_BEER_STYLE));
    }

    #[test]
    fn criteria_matches_inclusive() {
        let mut criteria = Criteria::default();
        assert!(criteria.matches(&TEST_BEER_STYLE));

        // Out of range values fails
        criteria.og = Some(-0.1);
        assert!(!criteria.matches(&TEST_BEER_STYLE));
        criteria.og = Some(1.1);
        assert!(!criteria.matches(&TEST_BEER_STYLE));

        // Inclusive values match
        criteria.og = Some(0.0);
        assert!(criteria.matches(&TEST_BEER_STYLE));
        criteria.og = Some(0.5);
        assert!(criteria.matches(&TEST_BEER_STYLE));
        criteria.og = Some(1.0);
        assert!(criteria.matches(&TEST_BEER_STYLE));
    }

    #[test]
    fn criteria_matches_multiple() {
        let mut criteria = Criteria::default();
        assert!(criteria.matches(&TEST_BEER_STYLE));

        // No criteria pass
        criteria.og = Some(-0.1);
        criteria.ibu = Some(1);
        assert!(!criteria.matches(&TEST_BEER_STYLE));

        // Some criteria pass
        criteria.og = Some(0.5);
        criteria.ibu = Some(1);
        assert!(!criteria.matches(&TEST_BEER_STYLE));
        criteria.og = Some(-0.1);
        criteria.ibu = Some(10);
        assert!(!criteria.matches(&TEST_BEER_STYLE));

        // All criteria pass
        criteria.og = Some(0.5);
        criteria.ibu = Some(10);
        assert!(criteria.matches(&TEST_BEER_STYLE));
    }

    #[test]
    fn criteria_matches_same_min_max() {
        let mut criteria = Criteria::default();
        assert!(criteria.matches(&TEST_BEER_STYLE));

        // We should be able to match a style where min and max are the same
        criteria.abv = Some(0.0);
        assert!(criteria.matches(&TEST_BEER_STYLE));
    }
}
