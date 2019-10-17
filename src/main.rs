#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

mod calculators;

fn main() {
    let mut app = App::new("RustyBeer")
                 .version("0.1")
                 .setting(AppSettings::ArgRequiredElseHelp);
    app = calculators::priming::add_subcommand(app);
    let matches = app.get_matches();
    
    calculators::priming::do_matches(matches);
}
