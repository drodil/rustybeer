#[macro_use]
extern crate clap;
mod calculators;

use clap::{App};

fn main() {

    let mut app = App::new("RustyBeer");
    app = calculators::priming::add_subcommand(app);
    app = calculators::abv::add_subcommand(app);
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("abv") {
        calculators::abv::do_matches(matches);
    }

}
