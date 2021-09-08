use rustybeer::calculators::abv::{calculate_abv, calculate_fg};
use rustybeer::conversions::RelativeDensityParser;
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Schema)]
pub struct AbvResponse {
    /// Alcohol by volume
    #[schema(example = "\"6.825000000000006\"")]
    abv: f64,
}

#[derive(Debug, Default, Deserialize, Schema)]
struct AbvRequest {
    /// Original gravity
    #[schema(example = "\"1.066\"")]
    og: String,
    /// Final gravity
    #[schema(example = "\"1.014\"")]
    fg: String,
}

#[post("/calculate/abv")]
#[openapi(
    id = "calculate.abv",
    description = "Calculates alcohol by volume (abv) in percentage from final and original gravity. To use plato or brix units for gravity, add the °P or °Bx after the value.",
    summary = "Calculate alcohol by volume",
    tags("calculator")
)]
pub fn abv(req: Json<AbvRequest>) -> Box<dyn Reply> {
    let value = req.into_inner();
    let ogrd = match RelativeDensityParser::parse(&value.og) {
        Ok(ogrd) => ogrd,
        Err(_) => return Box::new(http::StatusCode::BAD_REQUEST),
    };

    let fgrd = match RelativeDensityParser::parse(&value.fg) {
        Ok(fgrd) => fgrd,
        Err(_) => return Box::new(http::StatusCode::BAD_REQUEST),
    };

    Box::new(Json::from(AbvResponse {
        abv: calculate_abv(&ogrd, &fgrd),
    }))
}

#[derive(Debug, Default, Serialize, Schema)]
pub struct FgResponse {
    /// Final gravity
    #[schema(example = "\"1.014\"")]
    fg: f64,
}

#[derive(Debug, Default, Deserialize, Schema)]
struct FgRequest {
    /// Original gravity
    #[schema(example = "\"1.066\"")]
    og: String,
    /// Alcohol by volume
    #[schema(example = "\"6.825000000000006\"")]
    abv: f64,
}

#[post("/calculate/fg")]
#[openapi(
    id = "calculate.fg",
    description = "Calculates final gravity based on original gravity and wanted alcohol by volume. To use plato or brix units for gravity, add the °P or °Bx after the value.",
    summary = "Calculate final gravity",
    tags("calculator")
)]
pub fn fg(req: Json<FgRequest>) -> Box<dyn Reply> {
    let value = req.into_inner();
    let ogrd = match RelativeDensityParser::parse(&value.og) {
        Ok(ogrd) => ogrd,
        Err(_) => return Box::new(http::StatusCode::BAD_REQUEST),
    };

    Box::new(Json::from(FgResponse {
        fg: calculate_fg(&ogrd, value.abv),
    }))
}
