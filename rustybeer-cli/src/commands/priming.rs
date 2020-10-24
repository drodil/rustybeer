use rustybeer::calculators::priming::{calculate_co2, calculate_sugars};
use rustybeer_util::{
    conversions::{TemperatureParser, VolumeParser},
    measurements::{Temperature, Volume},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "priming")]
/// Beer Priming Calculator
pub struct PrimingOptions {
    #[structopt(short, long, parse(try_from_str = TemperatureParser::parse))]
    /// Temperature of beer with unit (C, F, K). Defaults to Celsius.
    temp: Temperature,

    #[structopt(short, long = "amount", parse(try_from_str = VolumeParser::parse))]
    /// Amount being packaged with unit (l, ml, gal, etc.). Defaults to liters.
    amount: Volume,

    #[structopt(short, long = "co2_volumes", default_value = "2.0")]
    /// Volumes of wanted CO2, depends on beer style (e.g. British Style Ales 1.5 to 2.0)
    co2_volumes: f64,
}

pub fn calculate_and_print(priming: PrimingOptions) {
    let fahrenheit = priming.temp.as_fahrenheit();
    let amount = priming.amount.as_litres();
    let co2_beer = calculate_co2(fahrenheit);
    let sugars = calculate_sugars(fahrenheit, amount, priming.co2_volumes);

    println!("Amount: {}l", amount);
    println!("Volumes of CO2: {}", priming.co2_volumes);
    println!("Temperature: {}C", fahrenheit);
    println!("CO2 in Beer: {:.2} volumes", co2_beer);
    println!("Priming Sugar Options:");
    for sugar in sugars.iter() {
        println!("{:>23}: {:.2} g", sugar.name, sugar.ratio);
    }
}
