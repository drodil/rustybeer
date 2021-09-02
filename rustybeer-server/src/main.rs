extern crate iron;
extern crate router;

mod handlers;

use iron::prelude::*;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.post("/abv", handlers::abv::handler, "abv");
    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
