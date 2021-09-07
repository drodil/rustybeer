pub use rustybeer::beer_styles::{BeerStyle, Criteria, BEER_STYLES};
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Schema)]
pub struct BeerQuery {
    /// Filter for beer style name, case insensitive
    name: Option<String>,

    /// Filter for original gravity
    og: Option<f32>,

    /// Filter for final gravity
    fg: Option<f32>,

    /// Filter for alcohol by volume
    abv: Option<f32>,

    /// Filter for international bittering units (IBU)
    ibu: Option<u8>,

    /// Filter for standard reference model color (SRM)
    color: Option<f32>,
}

// TODO: This is total copy of the beer style from rustybeer_util crate.
// If it's possible to find a way to reuse the same struct with different traits
// it should be done. But couldn't find a way.
#[derive(Debug, Serialize, Schema)]
pub struct BeerStyleResponse {
    /// Name of the beer style
    pub name: String,
    /// Minimum original gravity
    pub original_gravity_min: f32,
    /// Maximum original gravity
    pub original_gravity_max: f32,
    /// Minimum final gravity
    pub final_gravity_min: f32,
    /// Maximum final gravity
    pub final_gravity_max: f32,
    /// Minimum alcohol by volume
    pub abv_min: f32,
    /// Maximum alcohol by volume
    pub abv_max: f32,
    /// Minimum international bitterin units (IBU)
    pub ibu_min: u8,
    /// Maximum international bitterin units (IBU)
    pub ibu_max: u8,
    /// Minimum standard reference model color (SRM)
    pub color_srm_min: f32,
    /// Maximum standard reference model color (SRM)
    pub color_srm_max: f32,
    /// Description of the beer style
    pub description: String,
}

impl BeerStyleResponse {
    fn from_beerstyle(style: &BeerStyle) -> BeerStyleResponse {
        BeerStyleResponse {
            name: style.name.clone(),
            original_gravity_min: style.original_gravity_min,
            original_gravity_max: style.original_gravity_max,
            final_gravity_min: style.final_gravity_min,
            final_gravity_max: style.final_gravity_max,
            abv_min: style.abv_min,
            abv_max: style.abv_max,
            ibu_min: style.ibu_min,
            ibu_max: style.ibu_max,
            color_srm_min: style.color_srm_min,
            color_srm_max: style.color_srm_max,
            description: style.description.clone(),
        }
    }
}

#[get("/styles")]
#[openapi(
    id = "styles",
    description = "Lists beer styles that match optional query parameters",
    summary = "List beer styles"
)]
pub fn search(q: Query<BeerQuery>) -> Json<Vec<BeerStyleResponse>> {
    let query = q.into_inner();
    let criteria = Criteria {
        name: query.name,
        og: query.og,
        fg: query.fg,
        abv: query.abv,
        ibu: query.ibu,
        srm: query.color,
    };

    let resp: Vec<BeerStyleResponse> = BEER_STYLES
        .iter()
        .filter(|style| criteria.matches(style))
        .map(|style| BeerStyleResponse::from_beerstyle(&style))
        .collect();

    Json::from(resp)
}
