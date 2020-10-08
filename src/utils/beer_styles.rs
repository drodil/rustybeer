use once_cell::sync::Lazy;
use serde::Deserialize;

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
}

static BEER_STYLES_JSON: &str = include_str!("beer_styles.json");
pub static BEER_STYLES: Lazy<Vec<BeerStyle>> = Lazy::new(|| {
    serde_json::from_str(BEER_STYLES_JSON).expect("beer styles data could not be deserialised")
});

pub struct Criteria {
    pub og: Option<f32>,
    pub fg: Option<f32>,
    pub abv: Option<f32>,
    pub ibu: Option<u8>,
    pub srm: Option<f32>,
}

impl Criteria {
    pub fn matches(&self, style: &BeerStyle) -> bool {
        if let Some(og) = self.og {
            if !(og > style.original_gravity_min && og < style.original_gravity_max) {
                return false;
            }
        }
        if let Some(fg) = self.fg {
            if !(fg > style.final_gravity_min && fg < style.final_gravity_max) {
                return false;
            }
        }
        if let Some(abv) = self.abv {
            if !(abv > style.abv_min && abv < style.abv_max) {
                return false;
            }
        }
        if let Some(ibu) = self.ibu {
            if !(ibu > style.ibu_min && ibu < style.ibu_max) {
                return false;
            }
        }
        if let Some(srm) = self.srm {
            if !(srm > style.color_srm_min && srm < style.color_srm_max) {
                return false;
            }
        }

        true
    }
}
