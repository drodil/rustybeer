pub use rustybeer::hops::{get_hops, Criteria, Hop};
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Schema)]
pub struct HopQuery {
    /// Filter for hop name, case insensitive
    name: Option<String>,

    /// Filter for hop origin country, case insensitive
    country: Option<String>,

    /// Filter for alpha acid
    alpha_acid: Option<f64>,

    /// Filter for beta acid
    beta_acid: Option<f64>,

    /// Filter for hop purpose
    purpose: Option<String>,

    /// Filter for substituted hop
    substituted: Option<String>,
}

// TODO: This is total copy of the hop from rustybeer_util crate.
// If it's possible to find a way to reuse the same struct with different traits
// it should be done. But couldn't find a way.
#[derive(Debug, Serialize, Schema)]
pub struct HopResponse {
    /// Name of the hop
    pub name: String,
    /// Minimum alpha acid
    pub alpha_acid_min: f64,
    /// Maximum alpha acid
    pub alpha_acid_max: f64,
    /// Minimum beta acid
    pub beta_acid_min: f64,
    /// Maximum beta acid
    pub beta_acid_max: f64,
    /// Purpose(s) of the hop
    pub purpose: Vec<String>,
    /// Country of origin
    pub country: String,
    /// Description of the hop
    pub description: String,
    /// List of hops that this can be substituted with
    pub substitutions: Vec<String>,
}

impl HopResponse {
    fn from_hop(hop: &Hop) -> HopResponse {
        HopResponse {
            name: hop.name.clone(),
            alpha_acid_min: hop.alpha_acid_min,
            alpha_acid_max: hop.alpha_acid_max,
            beta_acid_min: hop.beta_acid_min,
            beta_acid_max: hop.beta_acid_max,
            purpose: hop.purpose.clone(),
            country: hop.country.clone(),
            description: hop.description.clone(),
            substitutions: hop.substitutions.clone(),
        }
    }
}

#[get("/hops")]
#[openapi(
    id = "hops",
    description = "Lists hops that match optional query parameters",
    summary = "List hops"
)]
pub fn search(q: Query<HopQuery>) -> Json<Vec<HopResponse>> {
    let query = q.into_inner();
    let criteria = Criteria {
        name: query.name,
        country: query.country,
        alpha_acid: query.alpha_acid,
        beta_acid: query.beta_acid,
        purpose: query.purpose,
        substituted: query.substituted,
    };

    let resp: Vec<HopResponse> = get_hops(Some(criteria))
        .iter()
        .map(|hop| HopResponse::from_hop(&hop))
        .collect();

    Json::from(resp)
}
