extern crate clap;
mod calculators;

use clap::{App};

fn main() {

    let mut app = App::new("RustyBeer");
    app = calculators::priming::add_subcommand(app);
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("priming") => println!("'priming' was used"),
        None        => println!("No subcommand was used"),
        _           => println!("Some other subcommand was used"),
    }
}
