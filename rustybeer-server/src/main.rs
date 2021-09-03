mod handlers;

use rweb::*;
use std::env;

/// Renders the homepage. TODO: Change this to return HTML
/// and link to /docs
#[get("/")]
fn default() -> String {
    return "Check the /docs".to_string();
}

fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        _ => 3000,
    }
}

#[tokio::main]
async fn main() {
    let (spec, filter) = openapi::spec().build(move || {
        handlers::abv::abv().or(handlers::abv::fg().or(handlers::beer_style::search()))
    });

    serve(filter.or(default()).or(openapi_docs(spec)))
        .run(([0, 0, 0, 0], get_port()))
        .await;
}
