extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};

fn calculate_co2(temp: i32) -> f64 {
    (3.0378 - 0.050062 * (temp as f64) + 0.00026555 * (temp.pow(2) as f64))
}

pub fn add_subcommand<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b>{
    app.subcommand(SubCommand::with_name("priming")
            .about("Beer Priming Calculator")   // The message displayed in "-h"
            .arg(Arg::with_name("temp")         // Priming own arguments
                    .long("temp")
                    .help("Temperature of Beer")
                    .required(true)
                    .takes_value(true))
        )
}

pub fn do_matches<'c>(matches: &ArgMatches<'c>){
    if let Some(ref sub_matches) = matches.subcommand_matches("priming") {
        let temprature = value_t!(sub_matches, "temp", i32).unwrap_or_else(|e| e.exit());
        println!("CO2: {}", calculate_co2(temprature));
    }
}
