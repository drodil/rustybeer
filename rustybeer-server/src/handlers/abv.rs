use rustybeer::calculators::abv::{calculate_abv, calculate_fg};
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

#[post("/calculate/abv")]
#[openapi(
    id = "calculate.abv",
    description = "Calculates alcohol by volume (abv) in percentage from final and original gravity",
    summary = "Calculate alcohol by volume",
    tags("calculator")
)]
pub fn abv(req: Json<AbvRequest>) -> Json<AbvResponse> {
    let value = req.into_inner();
    Json::from(AbvResponse {
        abv: calculate_abv(value.og, value.fg),
    })
}

#[derive(Debug, Default, Serialize, Schema)]
pub struct FgResponse {
    /// Final gravity
    fg: f32,
}

#[derive(Debug, Default, Deserialize, Schema)]
struct FgRequest {
    /// Original gravity
    og: f32,
    /// Alcohol by volume
    abv: f32,
}

#[post("/calculate/fg")]
#[openapi(
    id = "calculate.fg",
    description = "Calculates final gravity based on original gravity and wanted alcohol by volume",
    summary = "Calculate final gravity",
    tags("calculator")
)]
pub fn fg(req: Json<FgRequest>) -> Json<FgResponse> {
    let value = req.into_inner();
    Json::from(FgResponse {
        fg: calculate_fg(value.og, value.abv),
    })
}
