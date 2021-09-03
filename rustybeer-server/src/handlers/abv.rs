use rustybeer::calculators::abv::calculate_abv;
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Schema)]
pub struct AbvResponse {
    /// Alcohol by volume
    abv: f32,
}

#[derive(Debug, Default, Deserialize, Schema)]
struct AbvRequest {
    /// Original gravity
    og: f32,
    /// Final gravity
    fg: f32,
}

#[post("/abv")]
#[openapi(
    id = "abv",
    description = "Calculates alcohol by volume (abv) in percentage from final and original gravity",
    summary = "Calculate alcohol by volume",
    tags("calculator")
)]
pub fn handler(req: Json<AbvRequest>) -> Json<AbvResponse> {
    let value = req.into_inner();
    return Json::from(AbvResponse {
        abv: calculate_abv(value.og, value.fg),
    });
}
