use rustybeer::calculators::num_bottles::calculate_num_bottles;
use rustybeer::conversions::VolumeParser;
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Schema)]
struct NumBottlesRequest {
    /// Volume to bottle as string (for example "25l")
    #[schema(example = "\"25l\"")]
    volume: String,
}

#[derive(Debug, Default, Serialize, Schema)]
pub struct BottleResponse {
    /// Size of the bottle
    size: String,
    /// Number of bottles to contain the given volume
    num: i32,
}

impl BottleResponse {
    fn from_tuple(size: String, num: i32) -> BottleResponse {
        BottleResponse {
            size: size,
            num: num,
        }
    }
}

/// Calculates the number of different standard-size bottles needed to contain a given volume
#[post("/calculate/bottles")]
#[openapi(
    id = "calculate.bottles",
    description = "Calculates number of different standard-size bottles needed to contain a given volume",
    summary = "Calculate number of bottles",
    tags("calculator")
)]
pub fn bottles(req: Json<NumBottlesRequest>) -> Box<dyn Reply> {
    let value = req.into_inner();
    let unit = VolumeParser::parse(&value.volume);
    let volume = match unit {
        Ok(val) => val.as_milliliters(),
        Err(_) => return Box::new(http::StatusCode::BAD_REQUEST),
    };

    let bottles = calculate_num_bottles(volume);

    let mut resp: Vec<BottleResponse> = Vec::new();
    for bottle in bottles {
        resp.push(BottleResponse::from_tuple(bottle.0, bottle.1))
    }
    Box::new(Json::from(resp))
}
