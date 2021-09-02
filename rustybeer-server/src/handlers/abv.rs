extern crate iron;
extern crate rustc_serialize;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use rustybeer::calculators::abv::calculate_abv;
use std::io::Read;

#[derive(RustcEncodable)]
struct AbvResponse {
    abv: f32,
}

#[derive(RustcDecodable)]
struct AbvRequest {
    og: f32,
    fg: f32,
}

pub fn handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body
        .read_to_string(&mut payload)
        .expect("Failed to read request body");
    let incoming: AbvRequest = json::decode(&payload).unwrap();

    let response = AbvResponse {
        abv: calculate_abv(incoming.og, incoming.fg),
    };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}
