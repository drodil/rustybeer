use rustybeer::conversions::temp_to_map;
pub use rustybeer::yeasts::{Criteria, Yeast, YEASTS};
use rweb::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Schema)]
pub struct YeastQuery {
    /// Filter for producer, case insensitive
    pub company: Option<String>,
    /// Filter for yeast name, case insensitive
    pub name: Option<String>,
    /// Filter for yeast attenuation
    pub attenuation: Option<u8>,
    /// Filter for yeast optimal temperature
    pub temperature: Option<String>,
}

// TODO: This is total copy of the yeast from rustybeer_util crate.
// If it's possible to find a way to reuse the same struct with different traits
// it should be done. But couldn't find a way.
#[derive(Debug, Serialize, Schema)]
pub struct YeastResponse {
    /// Producer
    pub company: String,
    /// Name
    pub name: String,
    /// Id
    pub id: Option<String>,
    /// Minimum attenuation
    pub min_attenuation: Option<u8>,
    /// Maximum attenuation
    pub max_attenuation: Option<u8>,
    /// Attenuation level
    pub attenuation_level: Option<String>,
    /// Flocculation level
    pub flocculation: Option<String>,
    /// Minimum optimal temperature
    pub min_temp: Option<HashMap<String, f64>>,
    /// Maximum optimal temperature
    pub max_temp: Option<HashMap<String, f64>>,
    /// Alcohol tolerance
    pub alc_tolerance: Option<u8>,
    /// Alcohol tolerance level
    pub alc_tolerance_level: Option<String>,
}

impl YeastResponse {
    fn from_yeast(yeast: &Yeast) -> YeastResponse {
        YeastResponse {
            name: yeast.name.clone(),
            company: yeast.company.clone(),
            id: yeast.id.as_ref().map_or(None, |i| Some(i.to_string())),
            min_attenuation: yeast.min_attenuation,
            max_attenuation: yeast.max_attenuation,
            attenuation_level: yeast
                .attenuation_level
                .map_or(None, |a| Some(a.to_string())),
            flocculation: yeast.flocculation.map_or(None, |f| Some(f.to_string())),
            min_temp: yeast.min_temp.map_or(None, |t| Some(temp_to_map(t))),
            max_temp: yeast.max_temp.map_or(None, |t| Some(temp_to_map(t))),
            alc_tolerance: yeast.alc_tolerance,
            alc_tolerance_level: yeast
                .alc_tolerance_level
                .map_or(None, |a| Some(a.to_string())),
        }
    }
}

#[get("/yeasts")]
#[openapi(
    id = "yeasts",
    description = "Lists yeasts that match optional query parameters",
    summary = "List yeasts"
)]
pub fn search(q: Query<YeastQuery>) -> Json<Vec<YeastResponse>> {
    let query = q.into_inner();
    let criteria = Criteria {
        name: query.name,
        company: query.company,
        attenuation: query.attenuation,
        temperature: query.temperature,
    };

    let resp: Vec<YeastResponse> = YEASTS
        .iter()
        .filter(|yeast| criteria.matches(yeast))
        .map(|yeast| YeastResponse::from_yeast(&yeast))
        .collect();

    Json::from(resp)
}
