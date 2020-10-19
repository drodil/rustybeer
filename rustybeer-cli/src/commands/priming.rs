use clap::{value_t, App, Arg, ArgMatches};
use rustybeer::calculators::priming::{calculate_co2, calculate_sugars};
use rustybeer_util::conversions::{TemperatureBuilder, VolumeBuilder};

pub fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("priming")
                .about("Beer Priming Calculator")   // The message displayed in "-h"
                .arg(Arg::with_name("temp")         // Priming own arguments
                        .long("temp")
                        .short("t")
                        .help("Temperature of beer with unit (C, F, K). Defaults to Celsius.")
                        .default_value("20C")
                        .required(true)
                        .takes_value(true))
                .arg(Arg::with_name("amount")
                        .long("amount")
                        .short("a")
                        .help("Amount being packaged with unit (l, ml, gal, etc.). Defaults to liters.")
                        .default_value("25l")
                        .takes_value(true))
                .arg(Arg::with_name("co2_volumes")
                        .long("co2_volumes")
                        .short("c")
                        .help("Volumes of wanted CO2, depends on beer style (e.g. British Style Ales 1.5 to 2.0)")
                        .default_value("2.0")
                        .takes_value(true))
}

pub fn do_matches<'c>(matches: &ArgMatches<'c>) {
    if let Some(sub_matches) = matches.subcommand_matches("priming") {
        let temperature = value_t!(sub_matches, "temp", String).unwrap_or_else(|e| e.exit());
        let amount_str = value_t!(sub_matches, "amount", String).unwrap_or_else(|e| e.exit());
        let co2_volumes = value_t!(sub_matches, "co2_volumes", f64).unwrap_or_else(|e| e.exit());

        let fahrenheit = TemperatureBuilder::from_str(&temperature)
            .unwrap()
            .as_fahrenheit();
        let amount = VolumeBuilder::from_str(&amount_str).unwrap().as_litres();
        let co2_beer = calculate_co2(fahrenheit);
        let sugars = calculate_sugars(fahrenheit, amount, co2_volumes);

        println!("Amount: {}", amount_str);
        println!("Volumes of CO2: {}", co2_volumes);
        println!("Temperature: {}", temperature);
        println!("CO2 in Beer: {:.2} volumes", co2_beer);
        println!("Priming Sugar Options:");

        for sugar in sugars.iter() {
            println!("{:>23}: {:.2} g", sugar.name, sugar.ratio);
        }
    }
}
