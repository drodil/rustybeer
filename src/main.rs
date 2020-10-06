use clap::{App, AppSettings, ArgMatches};

mod calculators;
mod commands;
mod utils;

// Trait that all subcommands must implement
trait AppSubCommand {
    fn add_subcommand<'a, 'b>() -> App<'a, 'b>;
    fn do_matches<'c>(&self, matches: &ArgMatches<'c>);
}

fn main() {
    let app = App::new("RustyBeer")
        .version("0.1")
        .subcommand(commands::abv::Abv::add_subcommand())
        .subcommand(commands::boil_off::BoilOff::add_subcommand())
        .subcommand(commands::diluting::Diluting::add_subcommand())
        .subcommand(commands::priming::Priming::add_subcommand())
        .subcommand(commands::sg_correction::SgCorrection::add_subcommand())
        .subcommand(commands::beer_style::BeerStyleFinder::add_subcommand())
        .setting(AppSettings::ArgRequiredElseHelp);

    app.get_matches();
}
