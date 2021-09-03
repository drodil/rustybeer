mod handlers;

use rweb::*;

/// Renders the homepage. TODO: Change this to return HTML
/// and link to /docs
#[get("/")]
fn default() -> String {
    return "Check the /docs".to_string();
}

#[tokio::main]
async fn main() {
    let (spec, filter) = openapi::spec().build(move || handlers::abv::handler());

    serve(filter.or(default()).or(openapi_docs(spec)))
        .run(([0, 0, 0, 0], 3000))
        .await;
}
