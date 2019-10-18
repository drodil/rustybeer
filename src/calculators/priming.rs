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
                        .help("Temperature of Beer (in Celsius, only integers)")
                        .required(true)
                        .takes_value(true))
            )
    }

    fn do_matches<'c>(&self, matches: &ArgMatches<'c>){
        if let Some(ref sub_matches) = matches.subcommand_matches("priming") {
            let temprature = value_t!(sub_matches, "temp", i32).unwrap_or_else(|e| e.exit());
            println!("CO2: {}", self.calculate_co2(self.celsius_to_fahrenheit(temprature as f32)));
        }
    }
}

impl Priming {
    fn celsius_to_fahrenheit(&self, celsius: f32) -> f32 {
        (9.0/5.0) * celsius + 32.0
    }

    fn calculate_co2(&self, fahrenheit: f32) -> f32 {
        (3.0378 - 0.050062 * fahrenheit + 0.00026555 * fahrenheit.powf(2.0))
    }
}
