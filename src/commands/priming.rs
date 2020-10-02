extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};
use crate::AppSubCommand;
use futures::{future, Future, stream, Stream};
pub use crate::calculators::priming::Priming;

impl AppSubCommand for Priming {
    fn add_subcommand<'a, 'b>(&self, app: App<'a, 'b>) -> App<'a, 'b>{
        app.subcommand(SubCommand::with_name("priming")
                .about("Beer Priming Calculator")   // The message displayed in "-h"
                .arg(Arg::with_name("temp")         // Priming own arguments
                        .long("temp")
                        .short("t")
                        .help("Temperature of Beer (in Celsius, only integers)")
                        .required(true)
                        .takes_value(true))
                .arg(Arg::with_name("amount")
                        .long("amount")
                        .short("a")
                        .help("Amount Being Packaged (in Liters)")
                        .default_value("1")
                        .takes_value(true))
                .arg(Arg::with_name("co2_volumes")
                        .long("co2_volumes")
                        .short("c")
                        .help("Volumes of CO2, depends on beer style (e.g. British Style Ales 1.5 to 2.0)")
                        .default_value("2.0")
                        .takes_value(true))
            )
    }

    fn do_matches<'c>(&self, matches: &ArgMatches<'c>){
        if let Some(ref sub_matches) = matches.subcommand_matches("priming") {
            let temprature = value_t!(sub_matches, "temp", i32).unwrap_or_else(|e| e.exit());
            let amount = value_t!(sub_matches, "amount", f32).unwrap_or_else(|e| e.exit());
            let co2_volumes = value_t!(sub_matches, "co2_volumes", f32).unwrap_or_else(|e| e.exit());

            let fahrenheit = self.celsius_to_fahrenheit(temprature as f32);
            let co2_beer = self.calculate_co2(fahrenheit);
            let sugars = self.calculate_sugars(fahrenheit, amount, co2_volumes);

            println!("Amount (Liters): {}", amount);
            println!("Volumes of CO2: {}", co2_volumes);
            println!("Temprature (C): {}", temprature);
            println!("CO2 in Beer: {} volumes", co2_beer);
            println!("Priming Sugar Options:");
            sugars.for_each(|sugar| {
                println!("{:>23}: {:.2} g", sugar.name, sugar.ratio);
                Ok(())
            }).wait();
        }
    }
}

