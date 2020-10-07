#[macro_use]
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
        .subcommand(commands::beer_style::BeerStyleFinder::add_subcommand())
        .subcommand(commands::boil_off::BoilOff::add_subcommand())
        .subcommand(commands::diluting::Diluting::add_subcommand())
        .subcommand(commands::priming::Priming::add_subcommand())
        .subcommand(commands::sg_correction::SgCorrection::add_subcommand())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match app.subcommand_name() {
        Some(s) => match s {
            "abv" => {
                let abv = commands::abv::Abv;
                abv.do_matches(&app);
            }
            "beer_style" => {
                let beer_style = commands::beer_style::BeerStyleFinder;
                beer_style.do_matches(&app);
            }
            "boil_off" => {
                let boil_off = commands::boil_off::BoilOff;
                boil_off.do_matches(&app);
            }
            "diluting" => {
                let diluting = commands::diluting::Diluting;
                diluting.do_matches(&app);
            }
            "priming" => {
                let priming = commands::priming::Priming;
                priming.do_matches(&app);
            }
            "sg_correction" => {
                let sg_correction = commands::sg_correction::SgCorrection;
                sg_correction.do_matches(&app);
            }
            _ => println!("Not recognised subcommand"),
        },
        None => println!("A subcommand must be provided"),

    }
}
