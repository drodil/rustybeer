extern crate clap;
use clap::{App, AppSettings};

mod calculators;

fn main() {
    let mut app = App::new("RustyBeer")
                 .setting(AppSettings::ArgRequiredElseHelp);
    app = calculators::priming::add_subcommand(app);
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("priming") => calculators::priming::do_matches(matches)
    }
}
