#[macro_use]
extern crate clap;
use clap::{App, AppSettings, ArgMatches};

mod calculators;
mod commands;
mod utils;

// Trait that all subcommands must implement
trait AppSubCommand {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>;
    fn do_matches<'c>(&self, matches: &ArgMatches<'c>);
}

// List containing all subcommands
struct ListOfSubCommands {
    list: Vec<Box<dyn AppSubCommand>>,
}

impl ListOfSubCommands {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn push<S: AppSubCommand + 'static>(&mut self, command: S) -> &mut Self {
        self.list.push(Box::new(command));
        self
    }
}

fn main() {
    let mut app = App::new("RustyBeer")
        .version("0.1")
        .setting(AppSettings::ArgRequiredElseHelp);

    // Add subcommands here
    let mut commands = ListOfSubCommands::new();
    commands.push(commands::abv::Abv);
    commands.push(commands::boil_off::BoilOff);
    commands.push(commands::diluting::Diluting);
    commands.push(commands::priming::Priming);
    commands.push(commands::sg_correction::SgCorrection);
    commands.push(commands::beer_style::BeerStyleFinder);
    commands.push(commands::num_bottles::NumBottles);

    // Allow subcommands to add their own parameters
    for command in &commands.list {
        app = command.add_subcommand(app);
    }

    let matches = app.get_matches();

    // Allow subcommands to handle their own parameters
    for command in &commands.list {
        command.do_matches(&matches);
    }
}
