extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};
use crate::AppSubCommand;

pub struct Priming;

impl AppSubCommand for Priming {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        app.subcommand(SubCommand::with_name("priming")
                .about("Beer Priming Calculator")   // The message displayed in "-h"
                .arg(Arg::with_name("temp")         // Priming own arguments
                        .long("temp")
                        .help("Temperature of Beer")
                        .required(true)
                        .takes_value(true))
            )
    }

    fn do_matches<'c>(&self, matches: &ArgMatches<'c>){
        if let Some(ref sub_matches) = matches.subcommand_matches("priming") {
            let temprature = value_t!(sub_matches, "temp", i32).unwrap_or_else(|e| e.exit());
            println!("CO2: {}", self.calculate_co2(temprature));
        }
    }
}

impl Priming {
    fn calculate_co2(&self, temp: i32) -> f64 {
        (3.0378 - 0.050062 * (temp as f64) + 0.00026555 * (temp.pow(2) as f64))
    }
}
